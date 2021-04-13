use super::*;



pub(super) struct GIntScoreTreeIter<'a> {
    pub(super) number :&'a i64,
    pub(super) stage : u16,
}




impl<'a> Iterator for GIntScoreTreeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        
        
        
        None
    }
}


impl<'a> GIntScoreTreeIter<'a> {

}


