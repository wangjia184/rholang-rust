
use model::*;

use super::*;

#[derive(Debug)]
struct RecvRawBinding {
    names : Vec<RawName>,
    name : RawName,
    remainder : RawNameRemainder,
    
}

struct Receipt {
    raw_bindings : Vec<RecvRawBinding>,
    persistent : bool,
    peek : bool,
}

impl super::Normalizer {

    pub fn normalize_input(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {

        let receipt_ = unsafe { proc.u.pinput_.receipt_ };
        let proc_ = unsafe { proc.u.pinput_.proc_ };

        if receipt_ == 0 as bnfc::Receipt {
            return Err(CompiliationError::new_null_pointer("pinput_.receipt_"));
        }
        if proc_ == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("pinput_.proc_"));
        }

        // To handle the most common case where we can sort the binds because
        // they're from different sources, Each channel's list of patterns starts its free variables at 0.
        // We check for overlap at the end after sorting. We could check before, but it'd be an extra step.

        // We split this into parts. First we process all the sources, then we process all the bindings.
        
        match self.extract_receipt(unsafe { &*receipt_ })? {
            Receipt { raw_bindings, persistent, peek } => {
                let (vector, this_level_free, mut source_locally_free, source_connective_used) = self.process_source(raw_bindings, input)?;

                let processed_bindings = self.process_bindings(vector, input)?;
                // processed_bindings is type of  Vec<( ReceiveBind, Rc<DeBruijnLevelMap>, BitSet )>

                let bindings_free = processed_bindings.iter().fold( BitSet::new(), |mut bitset, (_,_, bs) | {
                    bitset.union_with(&bs); 
                    bitset 
                });

                
                // TODO : sort it

                // TODO: check if receive contains the same source channel

                let merged_free = processed_bindings.iter().fold( DeBruijnLevelMap::empty(), |mut level_map, ( _, sub_level_map, _ ) | {
                    let pairs = level_map.merge(&sub_level_map);
                    for (var_name, source_position, contra_position) in pairs {
                        self.syntax_errors.push(SyntaxError::new_unexpected_reuse_of_name_context_free( &var_name, source_position, contra_position));
                    }
                    level_map
                } );
                
                // the body part
                let bind_count = merged_free.count_no_wildcards();
                
                let updated_env = input.env.clone_then_absorb_free(&merged_free);
                let proc_visit_inputs = ProcVisitInputs {
                    par : Par::default(),
                    env : Rc::new(updated_env),
                    known_free : this_level_free,
                };
                let proc_visit_outputs = self.normalize_proc( proc_, &proc_visit_inputs)?;
                let binds = processed_bindings.into_iter().map( | (receive_bind, _, _) | receive_bind).collect();
                let connective_used = source_connective_used || proc_visit_outputs.par.connective_used;

                source_locally_free.union_with(&bindings_free);
                source_locally_free.union_with_option(proc_visit_outputs.par.locally_free.as_ref());

                let receive = Receive {
                    binds : binds,
                    body : Some(proc_visit_outputs.par),
                    persistent : persistent,
                    peek : peek,
                    bind_count : bind_count,
                    locally_free : Some(source_locally_free),
                    connective_used : connective_used,
                };
                ProcVisitOutputs {
                    par : input.par.clone_then_prepend_receive(receive),
                    known_free : proc_visit_outputs.known_free,
                }
            }
        };

        
        

        let outputs = ProcVisitOutputs {
            par : Par::default(),
            known_free : DeBruijnLevelMap::empty(),
        };

        Ok(outputs)
    }


    fn process_source(&mut self, raw_bindings : Vec<RecvRawBinding>, input: &ProcVisitInputs) 
        -> Result<(
            Vec<(Vec<RawName>, Par, RawNameRemainder)>,
            Rc<DeBruijnLevelMap>,
            BitSet,
            bool
        ), CompiliationError> 
    {
        let init_acc : Result<_, CompiliationError> = Ok((
            Vec::<(Vec<RawName>, Par, RawNameRemainder)>::new(),
            input.known_free.clone(),
            BitSet::new(), // locally_free
            false // is_connective_used
        ));

        let (mut vector, known_free, locally_free, connective_used) = raw_bindings.into_iter().fold( init_acc, | result, raw_binding| {
            result.and_then( | (mut vector, known_free, mut locally_free, connective_used) | {
                let name_visit_inputs = NameVisitInputs {
                    env : input.env.clone(),
                    known_free : known_free.clone(),
                };
                self.normalize_name( &raw_binding.name, &name_visit_inputs).map( |name_visit_outputs| {

                    let connective_used = connective_used || ParLocallyFree::connective_used(&name_visit_outputs.chan);
                    locally_free.union_with_option(ParLocallyFree::locally_free(&name_visit_outputs.chan, input.env.depth()).as_ref()); 
                    vector.insert( 0, ( raw_binding.names, name_visit_outputs.chan, raw_binding.remainder));

                    (vector, name_visit_outputs.known_free, locally_free, connective_used)
                })
            })
        })?;
        vector.reverse();
        Ok((vector, known_free, locally_free, connective_used))
    }

    fn process_bindings(&mut self, bindings : Vec<(Vec<RawName>, Par, RawNameRemainder)>, input: &ProcVisitInputs) 
        -> Result<
            Vec<(
                ReceiveBind,
                Rc<DeBruijnLevelMap>,
                BitSet,
            )>, CompiliationError> 
    {
        let mut list = vec![];
        for (names, par, remainder) in bindings {
            let init_acc : Result<_, CompiliationError> = Ok((
                Vec::<Par>::new(),
                Rc::new(DeBruijnLevelMap::empty()),
                BitSet::new()
            ));

            let (mut patterns, known_free, locally_free) = names.into_iter().fold( init_acc, | result, n| {
                result.and_then( | (mut vector, known_free, mut locally_free) | {
                    let name_visit_inputs = NameVisitInputs {
                        env : Rc::new(input.env.clone_then_push()),
                        known_free : known_free.clone(),
                    };
                    self.normalize_name( &n, &name_visit_inputs)
                    /* TODO : add syntax error check
                    .flatMap { res =>
                      failOnInvalidConnective(input.env.depth, res)
                        .fold(err => Sync[M].raiseError[NameVisitOutputs](err), _.pure[M])
                    }
                    */
                    .map( |name_visit_outputs| {
                        locally_free.union_with_option(ParLocallyFree::locally_free(&name_visit_outputs.chan, input.env.depth() + 1).as_ref()); 
                        vector.insert( 0, name_visit_outputs.chan);
                        (vector, known_free, locally_free)
                    })
                })
            })?;
            patterns.reverse();
            self.normalize_name_reminder( &remainder, known_free).map( |(var_option, kf)| {
                let rev_bind = ReceiveBind{
                    patterns : patterns,
                    source : Some(par),
                    remainder : var_option,
                    free_count : kf.count_no_wildcards()
                };
                list.push( (rev_bind, kf, locally_free) )
            })?;
        }

        Ok(list)
    }

    fn extract_receipt(&mut self, receipt : &RawReceipt) -> Result<Receipt, CompiliationError>{
        match receipt.kind {
            bnfc::Receipt__is_ReceiptLinear => {
                let receiptlinearimpl_ = unsafe { receipt.u.receiptlinear_.receiptlinearimpl_ };
                if receiptlinearimpl_ == 0 as bnfc::ReceiptLinearImpl {
                    return Err(CompiliationError::new_null_pointer("receiptlinear_.receiptlinearimpl_"));
                }
                let receiptlinearimpl = unsafe {*receiptlinearimpl_};

                match receiptlinearimpl.kind {
                    bnfc::ReceiptLinearImpl__is_LinearSimple => {
                        let mut listlinearbind_ = unsafe { receiptlinearimpl.u.linearsimple_.listlinearbind_ };
                        let mut raw_bindings = vec![];
                        while listlinearbind_ != 0 as bnfc::ListLinearBind {
                            let listlinearbind = unsafe { *listlinearbind_ };
                            raw_bindings.push(self.extract_linearbind(listlinearbind.linearbind_)?);
                            listlinearbind_ = listlinearbind.listlinearbind_;
                        }
                        Ok(Receipt {
                            raw_bindings : raw_bindings,
                            persistent : false,
                            peek : false,
                        })
                    },
                    _ => {
                        Err(CompiliationError::new_unrecognized_data( "receiptlinearimpl.kind", receiptlinearimpl.kind, receiptlinearimpl.line_number, receiptlinearimpl.char_number))
                    }
                }
                
            },
            bnfc::Receipt__is_ReceiptRepeated => {
                let receiptrepeatedimpl_ = unsafe { receipt.u.receiptrepeated_.receiptrepeatedimpl_ };
                if receiptrepeatedimpl_ == 0 as bnfc::ReceiptRepeatedImpl {
                    return Err(CompiliationError::new_null_pointer("receiptrepeated_.receiptrepeatedimpl_"));
                }
                let receiptrepeatedimpl = unsafe {*receiptrepeatedimpl_};

                match receiptrepeatedimpl.kind {
                    bnfc::ReceiptRepeatedImpl__is_RepeatedSimple => {
                        let mut listrepeatedbind_ = unsafe { receiptrepeatedimpl.u.repeatedsimple_.listrepeatedbind_ };
                        let mut raw_bindings = vec![];
                        while listrepeatedbind_ != 0 as bnfc::ListRepeatedBind {
                            let listrepeatedbind = unsafe { *listrepeatedbind_ };
                            raw_bindings.push(self.extract_repeatedbind(listrepeatedbind.repeatedbind_)?);
                            listrepeatedbind_ = listrepeatedbind.listrepeatedbind_;
                        }
                        Ok(Receipt {
                            raw_bindings : raw_bindings,
                            persistent : true,
                            peek : false,
                        })
                    },
                    _ => {
                        Err(CompiliationError::new_unrecognized_data( "receiptrepeatedimpl.kind", receiptrepeatedimpl.kind, receiptrepeatedimpl.line_number, receiptrepeatedimpl.char_number))
                    }
                }
            },
            bnfc::Receipt__is_ReceiptPeek => {
                let receiptpeekimpl_ = unsafe { receipt.u.receiptpeek_.receiptpeekimpl_ };
                if receiptpeekimpl_ == 0 as bnfc::ReceiptPeekImpl {
                    return Err(CompiliationError::new_null_pointer("receiptpeek_.receiptpeekimpl_"));
                }
                let receiptpeekimpl = unsafe {*receiptpeekimpl_};

                match receiptpeekimpl.kind {
                    bnfc::ReceiptPeekImpl__is_PeekSimple => {
                        let mut listpeekbind_ = unsafe { receiptpeekimpl.u.peeksimple_.listpeekbind_ };
                        let mut raw_bindings = vec![];
                        while listpeekbind_ != 0 as bnfc::ListPeekBind {
                            let listpeekbind = unsafe { *listpeekbind_ };
                            raw_bindings.push(self.extract_peekbind(listpeekbind.peekbind_)?);
                            listpeekbind_ = listpeekbind.listpeekbind_;
                        }
                        Ok(Receipt {
                            raw_bindings : raw_bindings,
                            persistent : false,
                            peek : true,
                        })
                    },
                    _ => {
                        Err(CompiliationError::new_unrecognized_data( "receiptpeekimpl.kind", receiptpeekimpl.kind, receiptpeekimpl.line_number, receiptpeekimpl.char_number))
                    }
                }
            },
            _ => {
                Err(CompiliationError::new_unrecognized_data( "pinput_.receipt_.kind", receipt.kind, receipt.line_number, receipt.char_number))
            }
        }
    }

    fn extract_linearbind(&mut self, linearbind_ : bnfc::LinearBind) -> Result<RecvRawBinding, CompiliationError> {
        if linearbind_ == 0 as bnfc::LinearBind {
            return Err(CompiliationError::new_null_pointer("linearbind_"));
        }
        let linearbind = unsafe { *linearbind_ };
        match linearbind.kind {
            bnfc::LinearBind__is_LinearBindImpl => {
                let listname_ = unsafe { linearbind.u.linearbindimpl_.listname_ };
                let nameremainder_ = unsafe { linearbind.u.linearbindimpl_.nameremainder_ };
                let name_ = unsafe { linearbind.u.linearbindimpl_.name_ };
                if listname_ == 0 as bnfc::ListName {
                    return Err(CompiliationError::new_null_pointer("linearbindimpl_.listname_"));
                }
                if nameremainder_ == 0 as bnfc::NameRemainder {
                    return Err(CompiliationError::new_null_pointer("linearbindimpl_.nameremainder_"));
                }
                if name_ == 0 as bnfc::Name {
                    return Err(CompiliationError::new_null_pointer("linearbindimpl_.name_"));
                }
                let vector = self.extract_listname(listname_)?;
                let name = unsafe { *name_ };
                Ok(RecvRawBinding{
                    names : vector,
                    name : name,
                    remainder : unsafe{ *nameremainder_ },
                })
            },
            _ => {
                Err(CompiliationError::new_unrecognized_data( "linearbind.kind", linearbind.kind, linearbind.line_number, linearbind.char_number))
            }
        }
    }

    
    


    fn extract_repeatedbind(&mut self, repeatedbind_ : bnfc::RepeatedBind) -> Result<RecvRawBinding, CompiliationError> {
        if repeatedbind_ == 0 as bnfc::RepeatedBind {
            return Err(CompiliationError::new_null_pointer("repeatedbind_"));
        }
        let repeatedbind = unsafe { *repeatedbind_ };
        match repeatedbind.kind {
            bnfc::RepeatedBind__is_RepeatedBindImpl => {
                let listname_ = unsafe { repeatedbind.u.repeatedbindimpl_.listname_ };
                let nameremainder_ = unsafe { repeatedbind.u.repeatedbindimpl_.nameremainder_ };
                let name_ = unsafe { repeatedbind.u.repeatedbindimpl_.name_ };
                if listname_ == 0 as bnfc::ListName {
                    return Err(CompiliationError::new_null_pointer("repeatedbindimpl_.listname_"));
                }
                if nameremainder_ == 0 as bnfc::NameRemainder {
                    return Err(CompiliationError::new_null_pointer("repeatedbindimpl_.nameremainder_"));
                }
                if name_ == 0 as bnfc::Name {
                    return Err(CompiliationError::new_null_pointer("repeatedbindimpl_.name_"));
                }
                let vector = self.extract_listname(listname_)?;
                let name = unsafe { *name_ };
                Ok(RecvRawBinding{
                    names : vector,
                    name : name,
                    remainder : unsafe{ *nameremainder_ },
                })
            },
            _ => {
                Err(CompiliationError::new_unrecognized_data( "repeatedbind.kind", repeatedbind.kind, repeatedbind.line_number, repeatedbind.char_number))
            }
        }
    }


    fn extract_peekbind(&mut self, peekbind_ : bnfc::PeekBind) -> Result<RecvRawBinding, CompiliationError> {
        if peekbind_ == 0 as bnfc::PeekBind {
            return Err(CompiliationError::new_null_pointer("peekbind_"));
        }
        let peekbind = unsafe { *peekbind_ };
        match peekbind.kind {
            bnfc::PeekBind__is_PeekBindImpl => {
                let listname_ = unsafe { peekbind.u.peekbindimpl_.listname_ };
                let nameremainder_ = unsafe { peekbind.u.peekbindimpl_.nameremainder_ };
                let name_ = unsafe { peekbind.u.peekbindimpl_.name_ };
                if listname_ == 0 as bnfc::ListName {
                    return Err(CompiliationError::new_null_pointer("peekbindimpl_.listname_"));
                }
                if nameremainder_ == 0 as bnfc::NameRemainder {
                    return Err(CompiliationError::new_null_pointer("peekbindimpl_.nameremainder_"));
                }
                if name_ == 0 as bnfc::Name {
                    return Err(CompiliationError::new_null_pointer("peekbindimpl_.name_"));
                }
                let vector = self.extract_listname(listname_)?;
                let name = unsafe { *name_ };
                Ok(RecvRawBinding{
                    names : vector,
                    name : name,
                    remainder : unsafe{ *nameremainder_ },
                })
            },
            _ => {
                Err(CompiliationError::new_unrecognized_data( "peekbind.kind", peekbind.kind, peekbind.line_number, peekbind.char_number))
            }
        }
    }

    fn extract_listname(&mut self, mut listname_ : bnfc::ListName) -> Result<Vec<RawName>, CompiliationError> {

        let mut vector : Vec<RawName> = vec![];

        while listname_ != 0 as bnfc::ListName {
            let listname = unsafe { *listname_ };

            let name_ = listname.name_;
            if name_ == 0 as bnfc::Name {
                return Err(CompiliationError::new_null_pointer("listname_.name_"));
            }

            vector.push(unsafe { *name_ });

            listname_ = listname.listname_;
        }

        Ok(vector)
    }
}