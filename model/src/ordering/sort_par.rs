use super::*;

impl<'a> Scorable<'a> for &'a Par {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        ParScoreTreeIter{
            term : self,
            stage : 0,
            sends_slice : &self.sends[..],
            receives_slice : &self.receives[..],
            exprs_slice : &self.exprs[..],
            news_slice : &self.news[..],
            unforgeable_slice : &self.unforgeables[..],
        }.into()
    }
}

impl<'a> From<ParScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: ParScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Par(inner)
    }
}


pub(super) struct ParScoreTreeIter<'a> {
    pub term : &'a Par,
    stage : u16,
    sends_slice : &'a [Send],
    receives_slice : &'a [Receive],
    exprs_slice : &'a [Expr],
    news_slice : &'a [New],
    unforgeable_slice : &'a [GUnforgeable],
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

    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::PAR as i64)))
    }

    fn sends_score<'b>(&'b mut self) -> Option<Node<'a>> {
        if !self.sends_slice.is_empty() {
            let sub_iter = self.sends_slice[0].score_tree_iter();
            self.sends_slice = &self.sends_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.receives_score()
        }
    }

    fn receives_score<'b>(&'b mut self) -> Option<Node<'a>> {
        if !self.receives_slice.is_empty() {
            let sub_iter = self.receives_slice[0].score_tree_iter();
            self.receives_slice = &self.receives_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.exprs_score()
        }
    }

    fn exprs_score<'b>(&'b mut self) -> Option<Node<'a>> {
        if !self.exprs_slice.is_empty() {
            let sub_iter = self.exprs_slice[0].score_tree_iter();
            self.exprs_slice = &self.exprs_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.news_score()
        }
    }

    fn news_score<'b>(&'b mut self) -> Option<Node<'a>> {
        if !self.news_slice.is_empty() {
            let sub_iter = self.news_slice[0].score_tree_iter();
            self.news_slice = &self.news_slice[1..];
            Some(Node::Children(sub_iter.into()))
        } else {
            self.stage += 1;
            self.matches_score()
        }
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
        if !self.unforgeable_slice.is_empty() {
            let sub_iter = self.unforgeable_slice[0].score_tree_iter();
            self.unforgeable_slice = &self.unforgeable_slice[1..];
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



impl Sortable for Par {
    fn sort(&mut self) {

        // first sort all nested struct
        for s in &mut self.sends { s.sort(); }
        for r in &mut self.receives { r.sort(); }
        for e in &mut self.exprs { e.sort(); }
        for n in &mut self.news { n.sort(); }
        // for m in &mut self.matches { m.sort(); }
        // for b in &mut self.bundles { b.sort(); }
        // for c in &mut self.connectives { c.sort(); }
        // for u in &mut self.unforgeables { u.sort(); }

        // then sort current struct
        self.sends.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
        self.receives.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
        self.exprs.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
        self.news.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
        // self.matches.sort_by( |left, right| {
        //     comparer(left.score_tree_iter(), right.score_tree_iter() )
        // });
        // self.bundles.sort_by( |left, right| {
        //     comparer(left.score_tree_iter(), right.score_tree_iter() )
        // });
        // self.connectives.sort_by( |left, right| {
        //     comparer(left.score_tree_iter(), right.score_tree_iter() )
        // });
        // self.unforgeables.sort_by( |left, right| {
        //     comparer(left.score_tree_iter(), right.score_tree_iter() )
        // });
    }
}



impl Sortable for Vec<Par> {
    fn sort(&mut self) {
        for p in &mut *self {
            p.sort();
        }
        self.sort_by( |left, right| {
            comparer(left.score_tree_iter(), right.score_tree_iter() )
        });
    }
}