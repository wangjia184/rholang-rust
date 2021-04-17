use blake3::Hasher as Blake3Hasher;
use blake3::Hash;
use prost::Message;
use model::*;
use bytes::BytesMut;

// Ignore the possibility of collisions and use hash as key for now
// With 256bits and one billion messages (p=109) then the probability is about 4.3*10-60.
pub trait StableHash {
    fn generate_hash_key(&self) -> Hash;
}


impl<T : Message> StableHash for T {
    fn generate_hash_key(&self) -> Hash {

        let mut buffer = BytesMut::new();

        let stopper = LocallyFreeEncodingStopper::default();
        self.encode(&mut buffer).unwrap();
        drop(stopper);
        
        let mut hasher = Blake3Hasher::new();
        hasher.update(&buffer);
        hasher.finalize()
    }
}


#[test]
fn locally_free_should_not_be_involved_when_generate_hash() {
    let mut par1 = Par::default();
    {
        let mut bitset = BitSet::new();
        bitset.insert(3);
        par1.locally_free = Some(bitset);
    }

    let mut par2 = Par::default();
    {
        let mut bitset = BitSet::new();
        bitset.insert(9);
        par2.locally_free = Some(bitset);
    }

    let mut par3 = Par::default();
    {
        par3.locally_free = None;
    }

    
    assert_eq!( par1.generate_hash_key(), par2.generate_hash_key());
    assert_eq!( par2.generate_hash_key(), par3.generate_hash_key());
}
