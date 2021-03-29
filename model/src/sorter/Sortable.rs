

pub trait Sortable<T> {
    fn sort_match(term : &T) -> ScoredTerm;
}


