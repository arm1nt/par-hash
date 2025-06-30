use crate::hasher::default::GenericHasher;
use crate::models::HashFunctionType;

mod default;

pub trait Hasher {

    fn update(&mut self, buffer: &mut [u8]);

    fn finalize(&mut self) -> Vec<u8>;

    fn compute_hash(&mut self, buffer: &mut Vec<u8>) -> Vec<u8>;
}

pub struct HasherFactory {
}

impl HasherFactory {

    pub fn get_instance(algorithm: &HashFunctionType) -> Box<dyn Hasher> {
        match algorithm {
            HashFunctionType::MD5 => GenericHasher::<::md5::Md5>::new(),
            HashFunctionType::SHA1 => GenericHasher::<::sha1::Sha1>::new(),
            HashFunctionType::SHA2_256 => GenericHasher::<::sha2::Sha256>::new(),
            HashFunctionType::SHA2_512 => GenericHasher::<::sha2::Sha512>::new(),
            HashFunctionType::SHA3_256 => GenericHasher::<::sha3::Sha3_256>::new(),
            HashFunctionType::SHA3_512 => GenericHasher::<::sha3::Sha3_512>::new(),
        }
    }
}
