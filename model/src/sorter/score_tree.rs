use std::cmp::{ PartialOrd, Ord, Ordering };
/**
  * Sorts the insides of the Par and ESet/EMap of the rholangADT
  *
  * A score tree is recursively built for each term and is used to sort the insides of Par/ESet/EMap.
  * For most terms, the current term type's absolute value based on the Score object is added as a Leaf
  * to the left most branch and the score tree built for the inside terms are added to the right.
  * The Score object is a container of constants that arbitrarily assigns absolute values to term types.
  * The sort order is total as every term type is assigned an unique value in the Score object.
  * For ground types, the appropriate integer representation is used as the base score tree.
  * For var types, the Debruijn level from the normalization is used.
  *
  * In order to sort an term, call [Type]SortMatcher.sortMatch(term)
  * and extract the .term  of the returned ScoredTerm.
  */



#[derive(Eq)]
pub enum ScoreAtom {
    IntAtom(i64),
    StringAtom(String),
    BytesAtom(Vec<u8>),
}

impl PartialEq for ScoreAtom {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd for ScoreAtom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreAtom {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ScoreAtom::IntAtom(num1), ScoreAtom::IntAtom(num2)) => num1.cmp(num2),
            (ScoreAtom::IntAtom(_), _) => Ordering::Less,
            (_, ScoreAtom::IntAtom(_)) => Ordering::Greater,
            (ScoreAtom::StringAtom(str1), ScoreAtom::StringAtom(str2)) => str1.cmp(str2),
            (ScoreAtom::StringAtom(_), _) => Ordering::Less,
            (_, ScoreAtom::StringAtom(_)) => Ordering::Greater,
            (ScoreAtom::BytesAtom(_), ScoreAtom::BytesAtom(_)) => unimplemented!("To be implemented"),
            _ => Ordering::Equal,
        }
    }
}


impl From<i64> for ScoreAtom {
    fn from(num: i64) -> Self {
        ScoreAtom::IntAtom(num)
    }
}

impl From<String> for ScoreAtom {
    fn from(s: String) -> Self {
        ScoreAtom::StringAtom(s)
    }
}


pub enum Tree {
    Leaf(ScoreAtom),
    Node(Vec<ScoreAtom>),
}
impl From<i64> for Tree {
    fn from(num: i64) -> Self {
        Tree::Leaf(ScoreAtom::IntAtom(num))
    }
}
impl From<String> for Tree {
    fn from(s: String) -> Self {
        Tree::Leaf(ScoreAtom::StringAtom(s))
    }
}

pub struct ScoredTerm { 
    score : Tree
}

impl ScoredTerm {
    pub fn compare(x : &Self, y: &Self) -> Ordering {
        ScoredTerm::compare_tree(&x.score, &y.score)
    }

    fn compare_tree(left : &Tree, right : &Tree) -> Ordering {
        match (left, right) {
            (Tree::Leaf(ref a), Tree::Leaf(ref b)) => a.cmp(b),
            (Tree::Leaf(_), Tree::Node(_)) => Ordering::Less,
            (Tree::Node(_), Tree::Leaf(_)) => Ordering::Greater,
            (Tree::Node(ref v1), Tree::Node(ref v2)) => {
                let mut iter1 = v1.iter();
                let mut iter2 = v2.iter();
                loop {
                    match (iter1.next(), iter2.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(ref a), Some(ref b)) => {
                            let order = a.cmp(b);
                            if order != Ordering::Equal {
                                return order;
                            }
                        }
                    }
                } 
            }
        }
    }
}