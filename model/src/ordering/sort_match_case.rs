use super::*;


impl<'a> Scorable<'a> for &'a MatchCase {
    #[inline]
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        MatchCaseScoreTreeIter{
            term : self,
            stage : 0,
        }.into()
    }
}

impl<'a> From<MatchCaseScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: MatchCaseScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::MatchCase(inner)
    }
}

pub(super) struct MatchCaseScoreTreeIter<'a> {
    pub term : &'a MatchCase,
    stage : u16,
}


impl<'a> Iterator for MatchCaseScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.pattern_score()
            },
            1 => {
                self.source_score()
            },
            _ => None
        }
    }
}


// Node(Seq(sortedPattern.score) ++ Seq(sortedBody.score))
impl<'a> MatchCaseScoreTreeIter<'a> {

  

    #[inline]
    fn pattern_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.pattern {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

    #[inline]
    fn source_score(&mut self) -> Option<Node<'a>> {

        self.stage += 1;
        if let Some(ref par) = self.term.source {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

   
}



impl Sortable for MatchCase {
    #[inline]
    fn sort(&mut self) {
        if let Some(ref mut p) = self.pattern {
            p.sort();
        }
        if let Some(ref mut s) = self.source {
            s.sort();
        }
    }
}
