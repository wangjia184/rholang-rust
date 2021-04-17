use std::cmp::{ Ord, Ordering };
use super::rho_types::*;


/**
  * Sorts the insides of the Par and ESet/EMap of the rholangADT
  *
  * The score tree is recursively built for each term
  * For most terms, the current term type's absolute value based on the Score object is added as a Leaf
  * to the left most branch and the score tree built for the inside terms are added to the right.
  * The Score object is a container of constants that arbitrarily assigns absolute values to term types.
  * The sort order is total as every term type is assigned an unique value in the Score object.
  * For ground types, the appropriate integer representation is used as the base score tree.
  * For var types, the Debruijn level from the normalization is used.
  *
  * The Scala edition works in a low efficient way, it builds the entrie tree first then compare the order
  * But in real world, we don't need traverse the entrie tree to make the decision
  * Thus in our Rust edition, Sortable type allows to traverse the tree via Iterator trait.
  * And in most cases we dont need build the entrie tree if the comparion interrupts
  */

trait Scorable<'a> {
    fn score_tree_iter(self) -> ScoreTreeIter<'a>;
}

pub trait Sortable{
    fn sort(&mut self);
}

#[derive(Eq)]
enum ScoreAtom<'s> {
    IntAtom(i64),
    StringAtom(&'s String),
    BytesAtom,
}


enum Node<'a> {
    Leaf(ScoreAtom<'a>),
    Children(ScoreTreeIter<'a>),
}


pub mod sort_par;
pub mod sort_send;
pub mod sort_receive;
pub mod sort_receive_bind;
pub mod sort_var;
pub mod sort_new;
pub mod sort_expression;
mod hash;
pub use hash::*;

mod sort_send_test;
mod sort_new_test;
mod sort_receive_test;







// To avoid Box<dyn trait> and heap allocation
// use enum here for polymorphism
enum ScoreTreeIter<'a>{
    Par(sort_par::ParScoreTreeIter<'a>),
    New(sort_new::NewScoreTreeIter<'a>),
    Send(sort_send::SendScoreTreeIter<'a>),
    Receive(sort_receive::ReceiveScoreTreeIter<'a>),
    ReceiveBind(sort_receive_bind::ReceiveBindScoreTreeIter<'a>),
    Var(sort_var::VarScoreTreeIter<'a>),
    Expr(sort_expression::ExprScoreTreeIter<'a>),
    ExprUnderlying(sort_expression::ExprUnderlyingIterWapper<'a>),
}

impl<'a> Iterator for ScoreTreeIter<'a> {
    type Item = Node<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        
        match self {
            ScoreTreeIter::Par(iter) => iter.next(),
            ScoreTreeIter::New(iter) => iter.next(),
            ScoreTreeIter::Send(iter) => iter.next(),
            ScoreTreeIter::Receive(iter) => iter.next(),
            ScoreTreeIter::ReceiveBind(iter) => iter.next(),
            ScoreTreeIter::Var(iter) => iter.next(),
            ScoreTreeIter::Expr(iter) => iter.next(),
            ScoreTreeIter::ExprUnderlying(iter) => iter.next(),
            _ => unreachable!("Bug! Some branch in ScoreTreeIter::score_tree_iter() is not implemented.")
        }
    }
}



// Depth-first traverse to compare two sortables
fn comparer<'a,'b>(mut left_iter : ScoreTreeIter<'a>, mut right_iter : ScoreTreeIter<'b>) -> Ordering {
    loop {
        match (left_iter.next(), right_iter.next()) {

            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(Node::Leaf(_)), Some(Node::Children(_)) ) => return Ordering::Less,
            (Some(Node::Children(_)), Some(Node::Leaf(_)) ) => return Ordering::Greater,
            (Some(Node::Leaf(ref left)), Some(Node::Leaf(ref right)) ) => {
                let order = left.cmp(right);
                if order != Ordering::Equal {
                    return order;
                }
            } 
            (Some(Node::Children(left)), Some(Node::Children(right)) ) => {
                let order = comparer(left, right);
                if order != Ordering::Equal {
                    return order;
                }
            }
        };
    }
    
}



impl Ord for ScoreAtom<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ScoreAtom::IntAtom(num1), ScoreAtom::IntAtom(num2)) => num1.cmp(num2),
            (ScoreAtom::IntAtom(_), _) => Ordering::Less,
            (_, ScoreAtom::IntAtom(_)) => Ordering::Greater,
            (ScoreAtom::StringAtom(str1), ScoreAtom::StringAtom(str2)) => str1.cmp(str2),
            (ScoreAtom::StringAtom(_), _) => Ordering::Less,
            (_, ScoreAtom::StringAtom(_)) => Ordering::Greater,
            (ScoreAtom::BytesAtom, ScoreAtom::BytesAtom) => todo!("Implement bytes"),
        }
    }
}

impl PartialOrd for ScoreAtom<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoreAtom<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}



/**
* Total order of all terms
*
* The general order is ground, vars, arithmetic, comparisons, logical, and then others
*/
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum Score {
// For things that are truly optional
    ABSENT = 0,

// Ground types
    BOOL           = 1,
    INT            = 2,
    STRING         = 3,
    URI            = 4,
    PRIVATE        = 5,
    ELIST          = 6,
    ETUPLE         = 7,
    ESET           = 8,
    EMAP           = 9,
    DEPLOYER_AUTH  = 10,
    DEPLOY_ID      = 11,
    SYS_AUTH_TOKEN = 12,

// Vars
    BOUND_VAR = 50,
    FREE_VAR  = 51,
    WILDCARD  = 52,
    REMAINDER = 53,

// Expr
    EVAR        = 100,
    ENEG        = 101,
    EMULT       = 102,
    EDIV        = 103,
    EPLUS       = 104,
    EMINUS      = 105,
    ELT         = 106,
    ELTE        = 107,
    EGT         = 108,
    EGTE        = 109,
    EEQ         = 110,
    ENEQ        = 111,
    ENOT        = 112,
    EAND        = 113,
    EOR         = 114,
    EMETHOD     = 115,
    EBYTEARR    = 116,
    EEVAL       = 117,
    EMATCHES    = 118,
    EPERCENT    = 119,
    EPLUSPLUS   = 120,
    EMINUSMINUS = 121,
    EMOD        = 122,

// Other
    QUOTE    = 203,
    CHAN_VAR = 204,

    SEND              = 300,
    RECEIVE           = 301,
    NEW               = 303,
    MATCH             = 304,
    BUNDLE_EQUIV      = 305,
    BUNDLE_READ       = 306,
    BUNDLE_WRITE      = 307,
    BUNDLE_READ_WRITE = 308,

    CONNECTIVE_NOT       = 400,
    CONNECTIVE_AND       = 401,
    CONNECTIVE_OR        = 402,
    CONNECTIVE_VARREF    = 403,
    CONNECTIVE_BOOL      = 404,
    CONNECTIVE_INT       = 405,
    CONNECTIVE_STRING    = 406,
    CONNECTIVE_URI       = 407,
    CONNECTIVE_BYTEARRAY = 408,

    PAR = 999,
}


