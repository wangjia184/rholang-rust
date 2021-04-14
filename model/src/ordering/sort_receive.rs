use super::*;


impl<'a> Scorable<'a> for &'a Receive {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        ReceiveScoreTreeIter{
            term : self,
            stage : 0,
            binds_slice : &self.binds,
        }.into()
    }
}

impl<'a> From<ReceiveScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    fn from(inner: ReceiveScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Receive(inner)
    }
}

pub(super) struct ReceiveScoreTreeIter<'a> {
    pub term : &'a Receive,
    stage : u16,
    binds_slice : &'a [ReceiveBind],
}


impl<'a> Iterator for ReceiveScoreTreeIter<'a> {
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
                self.peek_score()
            },
            3 => {
                self.bind_score()
            },
            4 => {
                self.body_score()
            },
            5 => {
                self.bind_count_score()
            },
            6 => {
                self.connective_used_score()
            },

            _ => None
        }
    }
}


// Node(
//     Score.RECEIVE,
//     Seq(Leaf(persistentScore), Leaf(peekScore)) ++
//       sortedBinds.map(_.score) ++
//       Seq(sortedBody.score) ++
//       Seq(Leaf(r.bindCount.toLong)) ++
//       Seq(Leaf(connectiveUsedScore)): _*
//   )
impl<'a> ReceiveScoreTreeIter<'a> {

    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::RECEIVE as i64)))
    }

    #[inline]
    fn persistent_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let persistent_score = if self.term.persistent {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }

    #[inline]
    fn peek_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let peek_score = if self.term.peek {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(peek_score)))
    }

    fn bind_score(&mut self) -> Option<Node<'a>> {

        if !self.binds_slice.is_empty() {
            let sub_iter = self.binds_slice[0].score_tree_iter();
            self.binds_slice = &self.binds_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.body_score()
        }

    }

    fn body_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.body {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            self.bind_count_score()
        }
    }

    #[inline]
    fn bind_count_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(self.term.bind_count as i64)))
    }

    #[inline]
    fn connective_used_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let persistent_score = if self.term.connective_used {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }


}



impl Sortable for Receive {
    fn sort(&mut self) {
        for b in &mut self.binds {
            b.sort();
        }
        if let Some(ref mut body) = self.body {
            body.sort();
        }
    }
}
