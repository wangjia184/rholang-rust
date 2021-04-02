use std::fmt;
use std::ffi::NulError;
use std::error::Error;
use super::*;


include!("RhoTypes.rs");
include!("SyntaxError.rs");
include!("SourcePosition.rs");
include!("CompiliationError.rs");
include!("ExecutionError.rs");

use var::*;

impl From<Expr> for Par {
    fn from(e: Expr) -> Self {
        let mut par = Par::default();
        if let Some(ref instance) = e.expr_instance {
            par.locally_free = ExprInstanceLocallyFree::locally_free(instance, 0);
            par.connective_used = ExprInstanceLocallyFree::connective_used(instance);
        }
        par.exprs.push(e);
        par
    }
}


impl Par {

    pub fn append(&mut self, other : &Par) {
        self.sends.extend(other.sends.iter().cloned());
        self.receives.extend(other.receives.iter().cloned());
        self.news.extend(other.news.iter().cloned());
        self.exprs.extend(other.exprs.iter().cloned());
        self.matches.extend(other.matches.iter().cloned());
        self.unforgeables.extend(other.unforgeables.iter().cloned());
        self.bundles.extend(other.bundles.iter().cloned());
        self.connectives.extend(other.connectives.iter().cloned());

        if other.locally_free.is_some() {
            if let Some(ref mut locally_free) = self.locally_free {
                locally_free.union_with_option(other.locally_free.as_ref());
            } else {
                self.locally_free = other.locally_free.clone();
            }
        }
        
        self.connective_used |= other.connective_used;
    }

    // create a par holding a variable
    fn new_with_var(var_instance : VarInstance) -> Self {
        Par::from(
            Expr {
                expr_instance: Some(
                    expr::ExprInstance::EVarBody(
                        EVar {
                            v : Some(
                                Var {
                                    var_instance : Some(var_instance)
                                }
                            )
                        }
                    )  
                )
            }
        )
    }


    // create a par holding a wildcard var
    pub fn new_wildcard_var() -> Self {
        let var = VarInstance::Wildcard(WildcardMsg::default());
        Par::new_with_var(var)
    }

    // create a par holding a bound variable
    pub fn new_bound_var(index : i32) -> Self {
        let var = VarInstance::BoundVar(index);
        Par::new_with_var(var)
    }

    // create a par holding a free variable
    pub fn new_free_var(level : i32) -> Self {
        let var = VarInstance::FreeVar(level);
        Par::new_with_var(var)
    }

    pub fn clone_then_prepend_receive(&self, r : Receive) -> Self {
        let mut cloned = self.clone();

        if let Some(new_bitset) = r.locally_free.as_ref() {
            cloned.locally_free = match cloned.locally_free {
                Some(mut bitset) => {
                    bitset.union_with(new_bitset);
                    Some(bitset)
                },
                None => {
                    Some(new_bitset.clone())
                }
            };
        }
        if r.connective_used {
            cloned.connective_used = true;
        }
        cloned.receives.insert(0, r);
        
        cloned
    }

    pub fn clone_then_prepend_new(&self, n : New) -> Self {
        let mut cloned = self.clone();

        if let Some(new_bitset) = n.locally_free.as_ref() {
            cloned.locally_free = match cloned.locally_free {
                Some(mut bitset) => {
                    bitset.union_with(new_bitset);
                    Some(bitset)
                },
                None => {
                    Some(new_bitset.clone())
                }
            };
        }
        if let Some(par) = n.p.as_ref() {
            if par.connective_used {
                cloned.connective_used = true;
            }
        }
        cloned.news.insert(0, n);
        
        cloned
    }


    pub fn clone_then_prepend_send(&self, s : RhoSend) -> Self {
        let mut cloned = self.clone();

        if let Some(new_bitset) = s.locally_free.as_ref() {
            cloned.locally_free = match cloned.locally_free {
                Some(mut bitset) => {
                    bitset.union_with(new_bitset);
                    Some(bitset)
                },
                None => {
                    Some(new_bitset.clone())
                }
            };
        }
        if s.connective_used {
            cloned.connective_used = true;
        }
        cloned.sends.insert(0, s);
        
        cloned
    }

    pub fn clone_then_prepend_expr(&self, e : Expr, depth : i32) -> Self {
        let mut cloned = self.clone();

        if let Some((locally_free, connective_used)) = e.expr_instance.as_ref().and_then(|instance| 
        {
            Some( 
                (
                    ExprInstanceLocallyFree::locally_free(instance, depth),
                    ExprInstanceLocallyFree::connective_used(instance),
                )
            )
        }) 
        {
            if let Some(new_bitset) = locally_free.as_ref() {
                cloned.locally_free = match cloned.locally_free {
                    Some(mut bitset) => {
                        bitset.union_with(new_bitset);
                        Some(bitset)
                    },
                    None => {
                        Some((*new_bitset).clone())
                    }
                };
            }
            if connective_used {
                cloned.connective_used = true;
            }
            
        } else {
            warn!("Expr.ExprInstance is None in clone_then_prepend_expr()");
        }
        
        cloned.exprs.insert(0, e);
        
        
        cloned
    }
}


impl NormalizeResult {
    pub fn new_fatal_error(err : &str) -> Self {
        let mut result = NormalizeResult::default();
        result.compiliation_error = Some(CompiliationError{
            kind : CompiliationErrorKind::SystemError as i32,
            message : format!("System error. {}", err),
            position : None,
            contra_position : None,
        });
        result
    }
}
impl From<NulError> for NormalizeResult {
    fn from(err: NulError) -> Self {
        NormalizeResult::new_fatal_error(&format!("std::ffi::NulError occured. {}", err))
    }
}
