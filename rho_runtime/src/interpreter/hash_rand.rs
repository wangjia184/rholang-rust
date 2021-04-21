
use rand::Rng;
use bytes::Bytes;

// TO BE IMPLEMENTED
pub struct HashRand {
}

impl HashRand {
    pub fn next() -> Bytes {
        let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
        Bytes::copy_from_slice(&random_bytes[..])
    }
}

