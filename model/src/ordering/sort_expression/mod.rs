use super::*;

use expr::ExprInstance;
mod sort_gint;
use sort_gint::GIntScoreTreeIter;
mod sort_var_instance;
use sort_var_instance::VarInstanceScoreTreeIter;

// To avoid Box<dyn trait> and heap allocation
// use enum here for polymorphism
pub(super) enum ExprUnderlyingIterWapper<'a>{
    GInt(GIntScoreTreeIter<'a>),
    VarInstance(VarInstanceScoreTreeIter<'a>),
}

impl<'a> Iterator for ExprUnderlyingIterWapper<'a> {
    type Item = Node<'a>;

    
    fn next(&mut self) -> Option<Self::Item> {
        
        match self {
            ExprUnderlyingIterWapper::GInt(iter) => iter.next(),
            ExprUnderlyingIterWapper::VarInstance(iter) => iter.next(),
            _ => unreachable!("Bug! Some branch in ExprInstanceScoreTreeIter::score_tree_iter() is not implemented.")
        }
    }
}


impl<'a> From<ExprUnderlyingIterWapper<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: ExprUnderlyingIterWapper<'a>) -> Self {
        ScoreTreeIter::ExprUnderlying(inner)
    }
}


impl<'a> Scorable<'a> for &'a Expr {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {

        match self.expr_instance {

            Some(ExprInstance::GInt(ref num)) => 
                GIntScoreTreeIter { number : num, stage : 0}.into(),

            Some(ExprInstance::EVarBody(EVar { v: Some(Var { var_instance : Some(ref var)}) })) => 
                ExprScoreTreeIter {
                    type_score : Some(ScoreAtom::IntAtom(Score::EVAR as i64)),
                    inner : Some(VarInstanceScoreTreeIter{ term : var, stage : 0 }.into())
                }.into(),
                

            _ => {
                panic!("Unhandled type {:?} in ExprScoreTreeIter", self)
            }
        }

        
    }
}


impl<'a> From<ExprScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: ExprScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Expr(inner)
    }
}

pub(super) struct ExprScoreTreeIter<'a> {
    type_score : Option<ScoreAtom<'a>>,
    inner : Option<ExprUnderlyingIterWapper<'a>>,
}


impl<'a> Iterator for ExprScoreTreeIter<'a> {
    type Item = Node<'a>;


    fn next(&mut self) -> Option<Self::Item> {
        if let Some(type_score) = self.type_score.take() {
            Some(Node::Leaf(type_score))
        } else {
            if let Some(inner) = self.inner.take() {
                Some(Node::Children(inner.into()))
            } else {
                None
            }
        }
    }
}


impl<'a> ExprScoreTreeIter<'a> {

}



impl Sortable for Expr {
    fn sort(&mut self) {

    }
}