use std::convert::TryInto;
use super::BitSet;
use super::*;
use super::rho_types::{ var::VarInstance, expr::ExprInstance };

    /** Return true if a connective (including free variables and wildcards) is
    *  used anywhere in {@code source}.
    *  @param source the object in question
    *  Specifically looks for constructions that make a pattern non-concrete.
    *  A non-concrete pattern cannot be viewed as if it were a term.
    */
pub trait HasLocallyFree<T> {
    fn connective_used(source : &T) -> bool;

    /** Returns a bitset representing which variables are locally free if the term
    *  is located at depth {@code depth}
    *  @param source the object in question
    *  @param depth pattern nesting depth
    *  This relies on cached values based on the actual depth of a term and will
    *  only return the correct answer if asked about the actual depth of a term.
    *  The reason we pass depth is that building the caches calls this API and for
    *  the few instances where we don't rely on the cache, we need to know the
    *  depth.
    *
    *  Depth is related to pattern nesting. A top level term is depth 0. A pattern
    *  in a top-level term is depth 1. A pattern in a depth 1 term is depth 2,
    *  etc.
    */
    fn locally_free(source : &T, depth : i32) -> Option<BitSet>;
}


pub struct ParLocallyFree {}

impl HasLocallyFree<Par> for ParLocallyFree {
    fn connective_used(p : &Par) -> bool {
        p.connective_used
    }

    fn locally_free(p : &Par, _depth : i32) -> Option<BitSet> {
        p.locally_free.clone()
    }
}


pub struct VarInstanceLocallyFree {}

impl HasLocallyFree<VarInstance> for VarInstanceLocallyFree {
    fn connective_used(v : &VarInstance) -> bool {
        match v {
            VarInstance::BoundVar(_) => false,
            VarInstance::FreeVar(_) => true,
            VarInstance::Wildcard(_) => true,
            //_ => false,
        }
    }

    fn locally_free(v : &VarInstance, depth : i32) -> Option<BitSet> {
        match v {
            VarInstance::BoundVar(index) if depth == 0 => {
                let mut bitset = BitSet::default();
                bitset.insert((*index as u64).try_into().unwrap());
                Some(bitset)
            },
            VarInstance::FreeVar(_) => None,
            VarInstance::Wildcard(_) => None,
            _ => None,
        }
    }
}


pub struct ExprInstanceLocallyFree {}


impl HasLocallyFree<ExprInstance> for ExprInstanceLocallyFree {
    fn connective_used(e : &ExprInstance) -> bool {
        match e {
            ExprInstance::GBool(_) => false,
            ExprInstance::GInt(_) => false,
            ExprInstance::GString(_) => false,
            ExprInstance::GUri(_) => false,
            ExprInstance::GByteArray(_) => false,
            ExprInstance::EListBody(e) => e.connective_used,
            ExprInstance::ETupleBody(e) => e.connective_used,
            ExprInstance::ESetBody(e) => e.connective_used,
            ExprInstance::EMapBody(e) => e.connective_used,

            ExprInstance::EVarBody(EVar {
                v : Some(
                    Var{
                        var_instance : Some(var)
                    }
                )
            }) => VarInstanceLocallyFree::connective_used(var),

            ExprInstance::ENotBody(ENot { p : Some(par) }) => par.connective_used,
            ExprInstance::ENegBody(ENeg { p : Some(par) }) => par.connective_used,
            ExprInstance::EMultBody(EMult { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EDivBody(EDiv { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EModBody(EMod { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EPlusBody(EPlus { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EMinusBody(EMinus { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::ELtBody(ELt { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::ELteBody(ELte { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EGtBody(EGt { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EGteBody(EGte { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EEqBody(EEq { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::ENeqBody(ENeq { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EAndBody(EAnd { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EOrBody(EOr { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,

            ExprInstance::EMethodBody(e) => e.connective_used,
            ExprInstance::EMatchesBody(EMatches { target : Some(t), pattern : _}) => t.connective_used,

            ExprInstance::EPercentPercentBody(EPercentPercent { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EPlusPlusBody(EPlusPlus { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,
            ExprInstance::EMinusMinusBody(EMinusMinus { p1 : Some(par1), p2 : Some(par2) }) => par1.connective_used || par2.connective_used,

            
            _ => unimplemented!("unsupported ExprInstance {:?} in ExprInstanceLocallyFree::connective_used", e),
        }
    }

    fn locally_free(e : &ExprInstance, depth : i32) -> Option<BitSet> {
        match e {
            ExprInstance::GBool(_) => None,
            ExprInstance::GInt(_) => None,
            ExprInstance::GString(_) => None,
            ExprInstance::GUri(_) => None,
            ExprInstance::GByteArray(_) => None,
            ExprInstance::EListBody(e) => e.locally_free.clone(),
            ExprInstance::ETupleBody(e) => e.locally_free.clone(),
            ExprInstance::ESetBody(e) => e.locally_free.clone(),
            ExprInstance::EMapBody(e) => e.locally_free.clone(),

            ExprInstance::EVarBody(EVar {
                v : Some(
                    Var{
                        var_instance : Some(var)
                    }
                )
            }) => VarInstanceLocallyFree::locally_free(var, depth),

            ExprInstance::ENotBody(ENot { p : Some(par) }) => par.locally_free.clone(),
            ExprInstance::ENegBody(ENeg { p : Some(par) }) => par.locally_free.clone(),
            ExprInstance::EMultBody(EMult { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EDivBody(EDiv { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EModBody(EMod { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EPlusBody(EPlus { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EMinusBody(EMinus { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::ELtBody(ELt { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::ELteBody(ELte { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EGtBody(EGt { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EGteBody(EGte { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EEqBody(EEq { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::ENeqBody(ENeq { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EAndBody(EAnd { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EOrBody(EOr { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),

            ExprInstance::EMethodBody(e) => e.locally_free.clone(),
            ExprInstance::EMatchesBody(EMatches { target : Some(t), pattern : _}) => t.locally_free.clone(),

            ExprInstance::EPercentPercentBody(EPercentPercent { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EPlusPlusBody(EPlusPlus { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            ExprInstance::EMinusMinusBody(EMinusMinus { p1 : Some(par1), p2 : Some(par2) }) => BitSet::union_both( par1.locally_free.as_ref(), par2.locally_free.as_ref()),
            _ => unimplemented!("unsupported ExprInstance {:?} in ExprInstanceLocallyFree::locally_free", e),
        }
    }
}


