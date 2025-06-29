use crate::hasher::md5::Md5Hasher;
use crate::models::HashFunctionType;

mod md5;

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
            HashFunctionType::MD5 => Md5Hasher::new(),
            _ => todo!()
        }
    }
}