use digest::Digest;
use crate::hasher::Hasher;
use crate::util::error_exit;

pub struct GenericHasher<D: Digest + 'static> {
    internal_hasher: Option<D>,
}

impl <D: Digest + 'static> GenericHasher<D> {
    pub fn new() -> Box<dyn Hasher> {
        Box::new(Self {
            internal_hasher: Some(D::new())
        })
    }
}

impl <D: Digest + 'static> Hasher for GenericHasher<D> {
    fn update(&mut self, buffer: &mut [u8]) {
        if let Some(ref mut hasher) = self.internal_hasher {
            hasher.update(buffer);
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
