use super::*;

impl<'a> Sortable<ParScoreTreeIter<'a>> for &'a Par {
    fn score_tree_iter(self) -> ParScoreTreeIter<'a> {
        ParScoreTreeIter{
            data : self,
            stage : 0,
        }
    }
}

struct ParScoreTreeIter<'a> {
    data : &'a Par,
    stage : u16,
}


impl Iterator for ParScoreTreeIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        


          
        match self.stage {
            
            _ => None
        }
    }
}