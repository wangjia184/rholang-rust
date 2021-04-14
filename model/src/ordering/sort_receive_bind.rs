use super::*;


impl<'a> Scorable<'a> for &'a ReceiveBind {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        ReceiveBindScoreTreeIter{
            term : self,
            stage : 0,
            patterns_slice : &self.patterns[..],
        }.into()
    }
}

impl<'a> From<ReceiveBindScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: ReceiveBindScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::ReceiveBind(inner)
    }
}



pub(super) struct ReceiveBindScoreTreeIter<'a> {
    pub term : &'a ReceiveBind,
    stage : u16,
    patterns_slice : &'a [Par],
}


impl<'a> Iterator for ReceiveBindScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.source_score()
            },
            
            1 => {
                self.patterns_score()
            },

            2 => {
                self.reminder_score()
            },

            _ => None
        }
    }
}


// Node(Seq(sortedChannel.score) ++ sortedPatterns.map(_.score) ++ Seq(sortedRemainder.score))
impl<'a> ReceiveBindScoreTreeIter<'a> {

    fn source_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.source {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            self.patterns_score()
        }
    }

    fn patterns_score(&mut self) -> Option<Node<'a>> {
        if !self.patterns_slice.is_empty() {
            let sub_iter = self.patterns_slice[0].score_tree_iter();
            self.patterns_slice = &self.patterns_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.reminder_score()
        }
    }

    fn reminder_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref remainder) = self.term.remainder {
            let sub_iter = remainder.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }
}




impl Sortable for ReceiveBind {
    fn sort(&mut self) {
        for p in &mut self.patterns {
            p.sort();
        }
        if let Some(ref mut source) = self.source {
            source.sort();
        }
    }
}
