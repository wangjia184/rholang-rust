

use super::super::bnfc;
use super::*;


impl super::Normalizer {

    pub fn normalize_ifelse(&mut self, target_proc : bnfc::Proc, true_proc : bnfc::Proc, false_proc : bnfc::Proc, input: &ProcVisitInputs)
          -> Result<ProcVisitOutputs, CompiliationError> 
    {

        if target_proc == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("target_proc"));
        }
        if true_proc == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("true_proc"));
        }

        let target_output = self.normalize_proc( target_proc, input)?;

        let true_branch_input = ProcVisitInputs {
            par : Par::default(),
            env : input.env.clone(),
            known_free : Rc::new(target_output.known_free),
        };
        let true_branch_output = self.normalize_proc( true_proc, &true_branch_input)?;

        let mut false_branch_par = None;
        let known_free;
        if false_proc != 0 as bnfc::Proc {
            let false_branch_input = ProcVisitInputs {
                par : Par::default(),
                env : input.env.clone(),
                known_free : Rc::new(true_branch_output.known_free),
            };
            let false_branch_output = self.normalize_proc( false_proc, &false_branch_input)?;
            known_free = false_branch_output.known_free;
            false_branch_par = Some(false_branch_output.par);
            
        } else {
            known_free = true_branch_output.known_free;
        }


        let mut locally_free = BitSet::union_both(target_output.par.locally_free.as_ref(), true_branch_output.par.locally_free.as_ref());
        let mut connective_used = target_output.par.connective_used || true_branch_output.par.connective_used;
        if let Some(p) = &false_branch_par {
            if let Some(bs) = &p.locally_free {
                if let Some(dest_bitset) = &mut locally_free {
                    dest_bitset.union_with(&bs);
                } else {
                    locally_free = Some(bs.clone());
                }
            }
            if p.connective_used {
                connective_used = true;
            }
        }
        
        

        let m = Match {
            target : Some(target_output.par),
            cases : vec![
                MatchCase {
                    pattern : Some({
                        let mut pattern_par = Par::default();
                        pattern_par.exprs.push(Expr {
                            expr_instance : Some(expr::ExprInstance::GBool(true))
                        });
                        pattern_par
                    }),
                    source : Some(true_branch_output.par),
                    free_count : 0,
                },
                MatchCase {
                    pattern : Some({
                        let mut pattern_par = Par::default();
                        pattern_par.exprs.push(Expr {
                            expr_instance : Some(expr::ExprInstance::GBool(false))
                        });
                        pattern_par
                    }),
                    source : false_branch_par,
                    free_count : 0,
                }
            ],
            locally_free : locally_free,
            connective_used : connective_used,
        };
 

        Ok(
            ProcVisitOutputs {
                par : input.par.clone_then_prepend_match(m), 
                known_free : known_free 
            }
        )
    }

    
}