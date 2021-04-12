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


pub enum ScoreAtom {
    IntAtom(u16),
    StringAtom(String),
    BytesAtom,
}


impl From<Score> for ScoreAtom {
    fn from(score : Score) -> Self {
        Self::IntAtom(score as u16)
    }
}

pub enum Node {
    Leaf(ScoreAtom),
    Children(Box<dyn Iterator<Item = Node>>),
}

pub trait Sortable<ITER> where ITER : Iterator<Item = Node> {
    fn score_tree_iter(&self) -> ITER;
}






impl Sortable<SendScoreTreeIter<'_>> for Send {
    fn score_tree_iter<'b>(&'b self) -> SendScoreTreeIter<'b> {
        SendScoreTreeIter{
            data : self,
            stage : 0,
        }
    }
}

struct SendScoreTreeIter<'a> {
    data : &'a Send,
    stage : u16,
}


impl Iterator for SendScoreTreeIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        

        // Node(List(
        //         Leaf(ScoreAtom(IntAtom(Score.SEND))),
        //         Leaf(ScoreAtom(IntAtom(0))),
        //         Node(List(
        //             Leaf(ScoreAtom(IntAtom(Score.PAR))),
        //             Node(List(
        //                 Leaf(ScoreAtom(IntAtom(Score.RECEIVE))), 
        //                 Leaf(ScoreAtom(IntAtom(0))), 
        //                 Leaf(ScoreAtom(IntAtom(0))), 
        //                 Node(List(
        //                     Leaf(ScoreAtom(IntAtom(Score.PAR))),
        //                     Leaf(ScoreAtom(IntAtom(0)))
        //                 )),
        //                 Leaf(ScoreAtom(IntAtom(0))),
        //                 Leaf(ScoreAtom(IntAtom(0)))
        //             )),
        //             Leaf(ScoreAtom(IntAtom(0)))
        //         )),
        //         Leaf(ScoreAtom(IntAtom(0)))
        // ))

        // sendScore = Node(
        //     Score.SEND,
        //     Seq(Leaf(persistentScore)) ++ Seq(sortedChan.score) ++ sortedData.map(_.score) ++ Seq(
        //       Leaf(connectiveUsedScore)
        //     ): _*
        //   )
        
          
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(Score::SEND.into()))
            },


            _ => None
        }
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