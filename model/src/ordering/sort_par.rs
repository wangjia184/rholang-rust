use super::*;

impl<'a> Sortable<'a, ParScoreTreeIter<'a>> for &'a mut Par {
    fn score_tree_iter(self) -> ParScoreTreeIter<'a> {
        ParScoreTreeIter{
            term : self,
            stage : 0,
            //sends_slice : &self.sends[..],
            //receives_slice : &self.receives[..],
        }
    }
}

pub struct ParScoreTreeIter<'a> {
    pub term : &'a mut Par,
    stage : u16,
    //sends_slice : &'a [Send],
    //receives_slice : &'a [Receive],
}


impl<'a> Iterator for ParScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        


          
        match self.stage {
            0 => {
                self.object_score()
            },
            1 => {
                self.sends_score()
            },
            2 => {
                self.receives_score()
            },
            3 => {
                self.exprs_score()
            },
            4 => {
                self.news_score()
            },
            5 => {
                self.matches_score()
            },
            6 => {
                self.bundles_score()
            },
            7 => {
                self.connectives_score()
            },
            8 => {
                self.unforgeables_score()
            },
            9 => {
                self.connective_used_score()
            },

            _ => None
        }
    }
}




// Node(
//     Score.PAR,
//     sends.map(_.score) ++
//       receives.map(_.score) ++
//       exprs.map(_.score) ++
//       news.map(_.score) ++
//       matches.map(_.score) ++
//       bundles.map(_.score) ++
//       connectives.map(_.score) ++
//       unforgeables.map(_.score) ++
//       Seq(Leaf(connectiveUsedScore)): _*
//   )
impl<'a> ParScoreTreeIter<'a> {

    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(Score::PAR.into()))
    }

    fn sends_score<'b>(&'b mut self) -> Option<Node<'a>> {
        // if !self.sends_slice.is_empty() {
        //     let sub_iter = self.sends_slice[0].score_tree_iter();
        //     self.sends_slice = &self.sends_slice[1..];
        //     Some(Node::Children(Box::new(sub_iter)))
        // } else {
            self.stage += 1;
            self.receives_score()
        //}
    }

    fn receives_score<'b>(&'b mut self) -> Option<Node<'a>> {
        // if !self.receives_slice.is_empty() {
        //     let sub_iter = self.receives_slice[0].score_tree_iter();
        //     self.receives_slice = &self.receives_slice[1..];
        //     Some(Node::Children(Box::new(sub_iter)))
        // } else {
            self.stage += 1;
            self.exprs_score()
        //}
    }

    fn exprs_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.news_score()
    }

    fn news_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.matches_score()
    }

    fn matches_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.bundles_score()
    }

    fn bundles_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.connectives_score()
    }

    fn connectives_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.unforgeables_score()
    }

    fn unforgeables_score<'b>(&'b mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.connective_used_score()
    }

    fn connective_used_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        let persistent_score = if self.term.connective_used {1} else {0};
        Some(Node::Leaf(ScoreAtom::IntAtom(persistent_score)))
    }
    
}
