use protobuf::RepeatedField;
use super::rho_types::*;
use super::*;
use bit_set::BitSet;

// A wrapper class for par
pub struct RhoPar{
    par : Par,
    
    // At this moment rust-protobuf does not support custom type
    // So locally free must be serialized as bytes while it should be represented in BitSet in memory
    // this member here stores BitSet in memory and will be serialized into par's bytes on saving
    pub locally_free : BitSet<u64>,
}


impl Default for RhoPar {
    fn default() -> Self { 
        RhoPar {
            par : Par::default(),
            locally_free : BitSet::default(),
        }
     }
}

impl RhoPar {

    // create a par holding a variable
    fn new_with_var(var : Var) -> RhoPar {
        let mut evar = EVar::new();
        evar.set_v(var);
        let mut expr = Expr::new();
        expr.set_e_var_body(evar);
        let mut par = Par::new();
        par.set_exprs(RepeatedField::from(vec![expr]));
        RhoPar {
            par : par,
            locally_free : BitSet::default(),
        }
    }

    // create a par holding a wildcard var
    pub fn new_wildcard_var() -> RhoPar {
        let mut var = Var::new();
        var.set_wildcard(Var_WildcardMsg::default());
        RhoPar::new_with_var(var)
    }

    // create a par holding a bound variable
    pub fn new_bound_var(index : i32) -> RhoPar {
        let mut var = Var::new();
        var.set_bound_var(index);
        RhoPar::new_with_var(var)
    }

    // create a par holding a free variable
    pub fn new_free_var(level : i32) -> RhoPar {
        let mut var = Var::new();
        var.set_free_var(level);
        RhoPar::new_with_var(var)
    }

    pub fn is_connective_used(&self) -> bool {
        self.par.connective_used
    }

    pub fn into_new(self) -> RhoNew {
        let mut rho_new = RhoNew::new();
        rho_new.set_p(self.par);
        rho_new
    }
}