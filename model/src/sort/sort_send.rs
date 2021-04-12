use super::*;


impl<'a> Sortable<SendScoreTreeIter<'a>> for &'a Send {
    fn score_tree_iter(self) -> SendScoreTreeIter<'a> {
        SendScoreTreeIter{
            term : self,
            stage : 0,
            data : &self.data[..],
        }
    }
}

struct SendScoreTreeIter<'a> {
    term : &'a Send,
    stage : u16,
    data : &'a [Par],
}


impl Iterator for SendScoreTreeIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        

        // Node(List(
        //         Leaf(ScoreAtom(IntAtom(Score.SEND))),
        //         Leaf(ScoreAtom(IntAtom(0))),
        //         Node(List(
        //             Leaf(ScoreAtom(IntAtom(Score.PAR))),
        //             Node(List(
        //                 Leaf(ScoreAtom(IntAtom(Score.RECEIVE))), 
        //                 Leaf(ScoreAtom(IntAtom(0))), 
        //                 Leaf(ScoreAtom(IntAtom(0))), 
        //                 Node(List(
        //                     Leaf(ScoreAtom(IntAtom(Score.PAR))),
        //                     Leaf(ScoreAtom(IntAtom(0)))
        //                 )),
        //                 Leaf(ScoreAtom(IntAtom(0))),
        //                 Leaf(ScoreAtom(IntAtom(0)))
        //             )),
        //             Leaf(ScoreAtom(IntAtom(0)))
        //         )),
        //         Leaf(ScoreAtom(IntAtom(0)))
        // ))
        
        
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
impl SendScoreTreeIter<'_> {

    fn object_score(&mut self) -> Option<Node> {
        self.stage += 1;
        Some(Node::Leaf(Score::SEND.into()))
    }

    fn persistent_score(&mut self) -> Option<Node> {
        self.stage += 1;
        let persistent_score = if self.term.persistent {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }

    fn channel_score(&mut self) -> Option<Node> {
        self.stage += 1;
        if let Some(par) = self.term.chan {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(Box::new(sub_iter)))
        } else {
            self.data_score()
        }
    }

    fn data_score<'a>(&'a mut self) -> Option<Node> {
        if !self.term.data.is_empty() {
            let sub_iter = self.data[0].score_tree_iter();
            self.data = &self.data[1..];
            Some(Node::Children(Box::new(sub_iter)))
        } else {
            self.stage += 1;
            self.connective_used_score()
        }
    }

    fn connective_used_score(&mut self) -> Option<Node> {
        self.stage += 1;
        let persistent_score = if self.term.connective_used {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }
}
