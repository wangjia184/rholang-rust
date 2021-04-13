use super::*;



pub(super) struct GIntScoreTreeIter<'a> {
    pub(super) number :&'a i64,
    pub(super) stage : u16,
}




impl<'a> Iterator for GIntScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        
        match self.stage {
            0 => {
                self.object_score()
            },

            1 => {
                self.number_score()
            },

            _ => None
        }
    }
}


impl<'a> GIntScoreTreeIter<'a> {
    #[inline]
    fn object_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(Score::INT as i64)))
    }

    #[inline]
    fn number_score(&mut self) -> Option<Node<'a>> {
        self.stage += 1;
        Some(Node::Leaf(ScoreAtom::IntAtom(*self.number)))
    }
}


