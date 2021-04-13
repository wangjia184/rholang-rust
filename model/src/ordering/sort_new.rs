use super::*;


impl<'a> Scorable<'a, NewScoreTreeIter<'a>> for &'a New {
    fn score_tree_iter(self) -> NewScoreTreeIter<'a> {
        NewScoreTreeIter{
            term : self,
            stage : 0,
            has_uri : !self.uri.is_empty(),
            uri : &self.uri[..],
        }
    }
}


pub(super) struct NewScoreTreeIter<'a> {
    pub term : &'a New,
    stage : u16,
    has_uri : bool,
    uri : &'a [String],
}


impl<'a> Iterator for NewScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.stage {
            0 => {
                self.object_score()
            },
            1 => {
                self.bind_count_score()
            },
            2 => {
                self.uri_score()
            },
            
            _ => None
        }
    }
}


// new Node(
//     Leaf(Score.NEW) +: ((Leaf(n.bindCount.toLong) +: uriScore) ++ injectionsScore :+ sortedPar.score)
//   )
impl<'a> NewScoreTreeIter<'a> {

    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::RECEIVE as i64)))
    }

    fn bind_count_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(self.term.bind_count as i64)))
    }

    fn uri_score(&mut self) -> Option<Node<'a>> {
        if self.has_uri {
            if !self.uri.is_empty() {
                let u = &self.uri[0];
                self.uri = &self.uri[1..];
                Some(Node::Leaf(ScoreAtom::StringAtom(u)))
            } else {
                self.stage += 1;
                self.injections_score()
            }
        }
        else {
            self.stage += 1;
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
        
    }


    fn injections_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        self.par_score()
    }

    fn par_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        None
    }

    

}



impl Sortable for New {
    fn sort(&mut self) {
        if let Some(ref mut p) = self.p {
            p.sort();
        }
        self.uri.sort();
    }
}
