use super::*;
use super::sort_var_instance::VarInstanceScoreTreeIter;
use expr::ExprInstance;
mod sort_gint; pub(super) use sort_gint::*;

impl<'a> Scorable<'a> for &'a Expr {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {

        match self.expr_instance {

            Some(ExprInstance::GInt(ref num)) => 
                GIntScoreTreeIter { number : num, stage : 0}.into(),

            Some(ExprInstance::EVarBody(EVar { v: Some(Var { var_instance : Some(ref var)}) })) => 
                ExprScoreTreeIter {
                    type_score : Some(ScoreAtom::IntAtom(Score::EVAR as i64)),
                    inner : Some(Box::new(VarInstanceScoreTreeIter{ term : var, stage : 0 }.into()))
                }.into(),
                

            _ => {
                panic!("Unhandled type {:?} in ExprScoreTreeIter", self)
            }
        }

        
    }
}


impl<'a> From<ExprScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    fn from(inner: ExprScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Expr(inner)
    }
}



pub(super) struct ExprScoreTreeIter<'a> {
    type_score : Option<ScoreAtom<'a>>,
    inner : Option<Box<ScoreTreeIter<'a>>>,
}


impl<'a> Iterator for ExprScoreTreeIter<'a> {
    type Item = Node<'a>;


    fn next(&mut self) -> Option<Self::Item> {
        if let Some(type_score) = self.type_score.take() {
            Some(Node::Leaf(type_score))
        } else {
            if let Some(inner) = self.inner.take() {
                Some(Node::Children(*inner))
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