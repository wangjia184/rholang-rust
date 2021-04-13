use super::*;


impl<'a> Scorable<'a, VarScoreTreeIter<'a>> for &'a Var {
    fn score_tree_iter(self) -> VarScoreTreeIter<'a> {
        VarScoreTreeIter{
            term : self,
            stage : 0,
        }
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

    fn var_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref var) = self.term.var_instance {
            let sub_iter = var.score_tree_iter();
            Some(Node::Children(Box::new(sub_iter)))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

}

