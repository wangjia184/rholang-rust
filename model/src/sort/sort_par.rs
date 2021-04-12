use super::*;

impl<'a> Sortable<'a, ParScoreTreeIter<'a>> for &'a Par {
    fn score_tree_iter(self) -> ParScoreTreeIter<'a> {
        ParScoreTreeIter{
            term : self,
            stage : 0,
        }
    }
}

pub struct ParScoreTreeIter<'a> {
    pub term : &'a Par,
    stage : u16,
}


impl<'a> Iterator for ParScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        


          
        match self.stage {
            
            _ => None
        }
    }
}