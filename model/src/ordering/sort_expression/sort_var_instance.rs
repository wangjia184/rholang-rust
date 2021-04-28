use super::*;

impl<'a> Scorable<'a> for &'a var::VarInstance {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        VarInstanceScoreTreeIter{
            term : self,
            stage : 0,
        }.into()
    }
}

impl<'a> From<VarInstanceScoreTreeIter<'a>> for ExprUnderlyingIterWapper<'a> {
    fn from(inner: VarInstanceScoreTreeIter<'a>) -> Self {
        ExprUnderlyingIterWapper::VarInstance(inner)
    }
}


impl<'a> From<VarInstanceScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    fn from(inner: VarInstanceScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::ExprUnderlying(inner.into())
    }
}






pub(in super::super) struct VarInstanceScoreTreeIter<'a> {
    pub(super) term : &'a var::VarInstance,
    pub(super) stage : u16,
}


impl<'a> Iterator for VarInstanceScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.type_score()
            },
            
            1 => {
                self.index_score()
            },
            
            _ => None
        }
    }
}


// case BoundVar(level) => Leaves(Score.BOUND_VAR, level.toLong)
// case FreeVar(level)  => Leaves(Score.FREE_VAR, level.toLong)
// case Wildcard(_)     => Leaves(Score.WILDCARD)

impl<'a> VarInstanceScoreTreeIter<'a> {

    fn type_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        match self.term {
            &var::VarInstance::BoundVar(_) => {
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::BOUND_VAR as i64)))
            },
            &var::VarInstance::FreeVar(_) => {
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::FREE_VAR as i64)))
            },
            &var::VarInstance::Wildcard(_) => {
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::WILDCARD as i64)))
            },
        }
    }

    fn index_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        match self.term {
            &var::VarInstance::BoundVar(i) => {
                Some(Node::Leaf(ScoreAtom::IntAtom(i as i64)))
            },
            &var::VarInstance::FreeVar(i) => {
                Some(Node::Leaf(ScoreAtom::IntAtom(i as i64)))
            },
            _ => {
                None
            }
        }
    }


}

