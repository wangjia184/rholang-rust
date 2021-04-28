use super::*;


impl<'a> Scorable<'a> for &'a Var {
    #[inline]
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        VarScoreTreeIter{
            term : self,
            stage : 0,
        }.into()
    }
}

impl<'a> From<VarScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: VarScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Var(inner)
    }
}

pub(super) struct VarScoreTreeIter<'a> {
    pub term : &'a Var,
    stage : u16,
}


impl<'a> Iterator for VarScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.var_score()
            },

            _ => None
        }
    }
}


// case BoundVar(level) => Leaves(Score.BOUND_VAR, level.toLong)
// case FreeVar(level)  => Leaves(Score.FREE_VAR, level.toLong)
// case Wildcard(_)     => Leaves(Score.WILDCARD)
// case Empty           => Leaf(Score.ABSENT)

impl<'a> VarScoreTreeIter<'a> {
    #[inline]
    fn var_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref var) = self.term.var_instance {
            let sub_iter = var.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

}

