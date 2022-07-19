#![doc = include_str!("../README.md")]

pub struct Signatures(phf::Map<u32, &'static str>);

impl Signatures {
    pub fn get(&self, signature: [u8; 4]) -> Option<&'static str> {
        self.0.get(&u32::from_be_bytes(signature)).copied()
    }
}

mod generated {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/signatures.rs"));
}

pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access() {
        assert_eq!(
            SIGNATURES.get([0xa9, 0x05, 0x9c, 0xbb]).unwrap(),
            "transfer(address,uint256)"
        );
    }
}
