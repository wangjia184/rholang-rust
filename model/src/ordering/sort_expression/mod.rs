use super::*;
use expr::ExprInstance;
mod sort_gint; use sort_gint::*;

impl<'a> Scorable<'a, ExprScoreTreeIter<'a>> for &'a Expr {
    fn score_tree_iter(self) -> ExprScoreTreeIter<'a> {

        match self.expr_instance {

            Some(ExprInstance::GInt(ref num)) => 
                ExprScoreTreeIter{
                    wapper : false,
                    type_score : None,
                    inner : Some(Box::new(GIntScoreTreeIter{ number : num, stage : 0 })),
                },

            Some(ExprInstance::EVarBody(EVar { v: Some(Var { var_instance : Some(ref var)}) })) => 
                ExprScoreTreeIter{
                    wapper : true,
                    type_score : Some(ScoreAtom::IntAtom(Score::EVAR as i64)),
                    inner : Some(Box::new(VarInstanceScoreTreeIter{ term : var, stage : 0 })),
                },
                

            _ => {
                panic!("Unhandled type {:?} in ExprScoreTreeIter", self)
            }
        }

        
    }
}




pub(super) struct ExprScoreTreeIter<'a> {
    wapper : bool,
    type_score : Option<ScoreAtom<'a>>,
    inner : Option<Box<dyn Iterator<Item = Node<'a>> + Sync + 'a>>,
}


impl<'a> Iterator for ExprScoreTreeIter<'a> {
    type Item = Node<'a>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.wapper {
            if let Some(type_score) = self.type_score.take() {
                Some(Node::Leaf(type_score))
            } else {
                if let Some(inner) = self.inner.take() {
                    Some(Node::Children(inner))
                } else {
                    None
                }
            }
        } else {
            if let Some(ref mut inner) = self.inner {
                inner.next()
            } 
            else {
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