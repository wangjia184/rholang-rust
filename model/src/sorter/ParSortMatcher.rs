
pub struct ParSortMatcher {}

impl ISortable<Par> for ParSortMatcher {
    fn sort_match(term : Par) -> ScoredTerm {
        ScoredTerm{
            term : Box::new(0),
            score : Tree::Leaf(ScoreAtom::IntAtom(9))
        }
    }
}