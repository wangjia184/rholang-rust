

pub trait ISortable<T> {
    fn sort_match(term : T) -> ScoredTerm;
}


pub struct Sortable {}

impl Sortable {
    fn sort_match<T>(term : T) -> ScoredTerm{
        ScoredTerm{
            term : Box::new(0),
            score : Tree::Leaf(ScoreAtom::IntAtom(9))
        }
    }
}
