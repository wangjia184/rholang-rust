use super::*;


impl<'a> Scorable<'a> for &'a Send {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        SendScoreTreeIter{
            term : self,
            stage : 0,
            data_slice : &self.data[..],
        }.into()
    }
}

impl<'a> From<SendScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: SendScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Send(inner)
    }
}


pub(super) struct SendScoreTreeIter<'a> {
    pub term : &'a Send,
    stage : u16,
    data_slice : &'a [Par],
}


impl<'a> Iterator for SendScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        
        match self.stage {
            0 => {
                self.object_score()
            },

            1 => {
                self.persistent_score()
            },

            2 => {
                self.channel_score()
            },

            3 => {
                self.data_score()
            },

            4 => {
                self.connective_used_score()
            }

            _ => None
        }
    }
}

// sendScore = Node(
//     Score.SEND,
//     Seq(Leaf(persistentScore)) ++ Seq(sortedChan.score) ++ sortedData.map(_.score) ++ Seq(
//       Leaf(connectiveUsedScore)
//     ): _*
//   )
impl<'a> SendScoreTreeIter<'a> {
    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::SEND as i64)))
    }

    #[inline]
    fn persistent_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let persistent_score = if self.term.persistent {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }

    fn channel_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.chan {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            warn!("SendScoreTreeIter::channel_score() returns None.");
            self.data_score()
        }
    }

    fn data_score<'b>(&'b mut self) -> Option<Node<'a>> {
        if !self.data_slice.is_empty() {
            let sub_iter = self.data_slice[0].score_tree_iter();
            self.data_slice = &self.data_slice[1..];
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



impl Sortable for Send {
    fn sort(&mut self) {
        if let Some(ref mut chan) = self.chan {
            chan.sort();
        }
        for item in &mut self.data {
            item.sort();
        }
    }
}