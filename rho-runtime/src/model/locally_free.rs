
use super::BitSet;
use super::*;

    /** Return true if a connective (including free variables and wildcards) is
    *  used anywhere in {@code source}.
    *  @param source the object in question
    *  Specifically looks for constructions that make a pattern non-concrete.
    *  A non-concrete pattern cannot be viewed as if it were a term.
    */
pub trait HasLocallyFree<T> {
    fn connective_used(source : &T) -> bool;

    /** Returns a bitset representing which variables are locally free if the term
    *  is located at depth {@code depth}
    *  @param source the object in question
    *  @param depth pattern nesting depth
    *  This relies on cached values based on the actual depth of a term and will
    *  only return the correct answer if asked about the actual depth of a term.
    *  The reason we pass depth is that building the caches calls this API and for
    *  the few instances where we don't rely on the cache, we need to know the
    *  depth.
    *
    *  Depth is related to pattern nesting. A top level term is depth 0. A pattern
    *  in a top-level term is depth 1. A pattern in a depth 1 term is depth 2,
    *  etc.
    */
    fn locally_free(source : &T, depth : i32) -> Option<&BitSet>;
}


pub struct ParLocallyFree {}

impl HasLocallyFree<Par> for ParLocallyFree {
    fn connective_used(p : &Par) -> bool {
        p.connective_used
    }

    fn locally_free(p : &Par, _depth : i32) -> Option<&BitSet> {
        p.locally_free.as_ref()
    }
}
