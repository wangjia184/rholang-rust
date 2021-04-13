use super::*;
use expr::ExprInstance;
mod sort_gint; use sort_gint::*;

impl<'a> Scorable<'a, ExprScoreTreeIter<'a>> for &'a Expr {
    fn score_tree_iter(self) -> ExprScoreTreeIter<'a> {

        let inner = match self.expr_instance {
            Some(ExprInstance::GInt(ref num)) => Box::new(GIntScoreTreeIter{
                number : num,
                stage : 0,
            }),
            _ => {
                unreachable!("score_tree_iter")
            }
        };

        ExprScoreTreeIter{
            term : self,
            inner : inner,
        }
    }
}




pub(super) struct ExprScoreTreeIter<'a> {
    pub term : &'a Expr,
    inner : Box<dyn Iterator<Item = Node<'a>> + Sync + 'a>,
}


impl<'a> Iterator for ExprScoreTreeIter<'a> {
    type Item = Node<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}


impl<'a> ExprScoreTreeIter<'a> {

}



impl Sortable for Expr {
    fn sort(&mut self) {

    }
}