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

        let _ = LocallyFreeEncodingStopper::default();
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer).unwrap();

        let mut hasher = Blake3Hasher::new();
        hasher.update(&buffer);
        hasher.finalize()
    }
}

#[test]
fn locally_free_should_not_be_involved_when_generate_hash() {
    let mut par1 = Par::default();
    {
        par1.sends.push({
            let mut send = Send::default();
            send.chan = Some(Par::default());
            send
        });
        let mut bitset = BitSet::new();
        bitset.insert(3);
        bitset.insert(4);
        bitset.insert(95);
        par1.locally_free = Some(bitset);
    }

    let mut par2 = Par::default();
    {
        par2.sends.push({
            let mut send = Send::default();
            send.chan = Some(Par::default());
            send
        });
    }

    assert_eq!( par1.generate_hash_key(), par2.generate_hash_key());
}