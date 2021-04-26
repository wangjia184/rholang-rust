use super::*;


impl<'a> Scorable<'a> for &'a Match {
    #[inline]
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        MatchScoreTreeIter{
            term : self,
            stage : 0,
            cases : &self.cases,
        }.into()
    }
}

impl<'a> From<MatchScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: MatchScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Match(inner)
    }
}

pub(super) struct MatchScoreTreeIter<'a> {
    pub term : &'a Match,
    stage : u16,
    cases : &'a [MatchCase],
}


impl<'a> Iterator for MatchScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.object_score()
            },
            1 => {
                self.target_score()
            },
            2 => {
                self.cases_score()
            },
            3 => {
                self.connective_used_score()
            },
            _ => None
        }
    }
}


// Node(
//     Score.MATCH,
//     Seq(sortedValue.score) ++ scoredCases.map(_.score) ++ Seq(Leaf(connectiveUsedScore)): _*
//   )
impl<'a> MatchScoreTreeIter<'a> {

    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::MATCH as i64)))
    }

    #[inline]
    fn target_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.target {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

    #[inline]
    fn cases_score(&mut self) -> Option<Node<'a>> {

        if !self.cases.is_empty() {
            let sub_iter = self.cases[0].score_tree_iter();
            self.cases = &self.cases[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.connective_used_score()
        }

    }

    #[inline]
    fn connective_used_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let persistent_score = if self.term.connective_used {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }

}



impl Sortable for Match {
    #[inline]
    fn sort(&mut self) {
        if let Some(target) = &mut self.target {
            target.sort();
        }
        for case in &mut self.cases {
            case.sort();
        }
    }
}
