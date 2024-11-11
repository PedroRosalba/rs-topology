use sha2::{Sha256, Digest};
pub struct HashableVec(Vec<u8>);

impl HashableVec {
    pub fn new(data: Vec<u8>) -> Self {
        HashableVec(data)
    }

    pub fn hash_sha256(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.0); 
        hasher.finalize().to_vec()

    }
}

