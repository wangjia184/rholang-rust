
use blake3::{ Hash, Hasher };
use super::*;

// The are not printable text
static INT_ATOM : &'static [u8] = &[0xFF, 8/*std::mem::size_of::<i64>*/];
static TEXT_ATOM: &'static [u8] = &[0xFE];
static BYTES_ATOM: &'static [u8] = &[0xFD];
static CHILD_START: &'static [u8] = &[0xFC];
static CHILD_END: &'static [u8] = &[0xFB];

pub trait Blake3Hashable {
    fn blake3_hash(&self) -> Hash;
}

impl Blake3Hashable for Par {
    
    fn blake3_hash(&self) -> Hash {

        fn traverse<'a>(mut iter : ScoreTreeIter<'a>, depth : i32, hasher: &mut Hasher) {
            loop {
                match iter.next() {
        
                    None => return,
                    Some(Node::Leaf(ScoreAtom::IntAtom(num))) => {
                        hasher.update(INT_ATOM);
                        hasher.update(&num.to_le_bytes());
                    },
                    Some(Node::Leaf(ScoreAtom::StringAtom(text))) => {
                        hasher.update(TEXT_ATOM);
                        hasher.update(&text.len().to_le_bytes());
                        hasher.update(text.as_bytes());
                    },
                    Some(Node::Leaf(ScoreAtom::BytesAtom)) => {
                        hasher.update(BYTES_ATOM);
                        todo!("Followed length and content")
                    },
                    Some(Node::Children(child_iter)) => {
                        hasher.update(CHILD_START);
                        hasher.update(&depth.to_le_bytes());
                        traverse(child_iter, depth + 1, hasher);
                        hasher.update(CHILD_END);
                    },
                };
            }
            
            
        }

        let mut hasher = Hasher::default();
        traverse(self.score_tree_iter(), 0, &mut hasher);
        hasher.finalize()
    }
}




#[test]
fn locally_free_should_not_be_involved_when_generate_hash() {
    let mut par1 = Par::default();
    {
        par1.sends.push({
            let mut send = Send::default();
            send.chan = Some({
                let mut p = Par::default();
                p.exprs.push(Expr {
                    expr_instance: Some(expr::ExprInstance::GInt(3))
                });
                p
            });
            send
        });
        let mut bitset = crate::BitSet::new();
        bitset.insert(3);
        par1.locally_free = Some(bitset);
    }

    let mut par2 = Par::default();
    {
        par2.sends.push({
            let mut send = Send::default();
            send.chan = Some({
                let mut p = Par::default();
                p.exprs.push(Expr {
                    expr_instance: Some(expr::ExprInstance::GInt(2))
                });
                p
            });
            send
        });
        let mut bitset = crate::BitSet::new();
        bitset.insert(9);
        par2.locally_free = Some(bitset);
    }

    let mut par3 = Par::default();
    {
        par3.sends.push({
            let mut send = Send::default();
            send.chan = Some({
                let mut p = Par::default();
                p.exprs.push(Expr {
                    expr_instance: Some(expr::ExprInstance::GInt(3))
                });
                p
            });
            send
        });
        par3.locally_free = None;
    }

    
    assert_ne!( par1.blake3_hash(), par2.blake3_hash());
    assert_eq!( par1.blake3_hash(), par3.blake3_hash());
}

