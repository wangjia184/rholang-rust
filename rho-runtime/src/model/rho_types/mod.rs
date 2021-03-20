use super::*;

include!("rho_types.rs");

use var::*;

impl Par {
    // create a par holding a variable
    fn new_with_var(var_instance : VarInstance) -> Self {
        let evar = Expr {
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
        };
        
        let mut par = Par::default();
        par.exprs.push(evar);
        
        par
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