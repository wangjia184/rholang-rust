use super::*;
use expr::ExprInstance;
mod sort_gint; use sort_gint::*;

impl<'a> Scorable<'a, ExprScoreTreeIter<'a>> for &'a Expr {
    fn score_tree_iter(self) -> ExprScoreTreeIter<'a> {

        match self.expr_instance {

            Some(ExprInstance::GInt(ref num)) => 
                ExprScoreTreeIter{
                    no_wrapper : true,
                    inner : Some(Box::new(GIntScoreTreeIter{ number : num, stage : 0 })),
                },

            Some(ExprInstance::EVarBody(EVar { v: Some(Var { var_instance : Some(ref var)}) })) => 
                ExprScoreTreeIter{
                    no_wrapper : true,
                    inner : Some(Box::new(VarInstanceScoreTreeIter{ term : var, stage : 0 })),
                },
                

            _ => {
                panic!("Unhandled type {:?} in ExprScoreTreeIter", self)
            }
        }

        
    }
}




pub(super) struct ExprScoreTreeIter<'a> {
    no_wrapper : bool, // do not generate wrapper, keep the same structure as Scala code
    inner : Option<Box<dyn Iterator<Item = Node<'a>> + Sync + 'a>>,
}


impl<'a> Iterator for ExprScoreTreeIter<'a> {
    type Item = Node<'a>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.no_wrapper {
            
            if let Some(inner) = &mut self.inner {
                if let Some(node) = inner.next() {
                    Some(node)
                } else {
                    self.inner = None;
                    None
                }
            } else {
                None
            }
        } else {
            if let Some(inner) = self.inner.take() {
                Some(Node::Children(Box::new(inner)))
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