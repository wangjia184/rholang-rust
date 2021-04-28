use super::*;


impl<'a> Scorable<'a> for &'a New {
    #[inline]
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        let mut injections : Vec<&Par> = self.injections.values().collect();
        injections.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
        NewScoreTreeIter{
            term : self,
            stage : 0,
            has_uri : !self.uri.is_empty(),
            uri : &self.uri[..],
            injection_index : 0,
            injections : injections,
        }.into()
    }
}

impl<'a> From<NewScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: NewScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::New(inner)
    }
}

pub(super) struct NewScoreTreeIter<'a> {
    pub term : &'a New,
    stage : u16,
    has_uri : bool,
    uri : &'a [String],
    injection_index : usize,
    injections : Vec<&'a Par>,
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
            3 => {
                self.injections_score()
            },
            4 => {
                self.par_score()
            },
            _ => None
        }
    }
}


// new Node(
//     Leaf(Score.NEW) +: ((Leaf(n.bindCount.toLong) +: uriScore) ++ injectionsScore :+ sortedPar.score)
//   )
impl<'a> NewScoreTreeIter<'a> {

    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::NEW as i64)))
    }

    #[inline]
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
        if !self.injections.is_empty() {
            if self.injection_index < self.injections.len() {
                let sub_iter = self.injections[self.injection_index].score_tree_iter();
                self.injection_index += 1;
                Some(Node::Children(sub_iter.into()))
            } else {
                self.stage += 1;
                self.par_score()
            }
        }
        else {
            self.stage += 1;
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }

    #[inline]
    fn par_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        if let Some(ref par) = self.term.p {
            let sub_iter = par.score_tree_iter();
            Some(Node::Children(sub_iter.into()))
        } else {
            Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
        }
    }
}



impl Sortable for New {
    #[inline]
    fn sort(&mut self) {
        if let Some(ref mut p) = self.p {
            p.sort();
        }
        self.uri.sort();
    }
}
