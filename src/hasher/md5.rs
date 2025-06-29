use digest::Digest;
use md5::Md5;
use crate::hasher::Hasher;
use crate::util::error_exit;

pub struct Md5Hasher {
    internal_hasher: Option<Md5>
}

impl Md5Hasher {
    /// Create a new instance of this MD5 hasher implementation
    pub fn new() -> Box<dyn Hasher> {
        Box::new(Md5Hasher {
            internal_hasher: Some(Md5::new())
        })
    }
}

impl Hasher for Md5Hasher {

    fn update(&mut self, buffer: &mut [u8]) {
        if let Some(ref mut hasher) = self.internal_hasher {
            Digest::update(hasher, buffer);
        }
    }

    fn finalize(&mut self) -> Vec<u8> {
        let hasher = self.internal_hasher.take().unwrap_or_else(|| {
            error_exit(Some("Hasher already finalized".to_string()));
        });

        hasher.finalize().to_vec()
    }

    fn compute_hash(&mut self, buffer: &mut Vec<u8>) -> Vec<u8> {
        for chunk in buffer.chunks_mut(8192) {
            self.update(chunk);
        }

        self.finalize()
    }

}
