#[cfg(test)]
use anyhow::Result;

pub trait Seed {
    fn as_bytes(&self) -> &[u8];
}

pub struct SeedImpl {
    bytes: Vec<u8>,
}

impl Seed for SeedImpl {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
pub fn from_bytes(bytes: Vec<u8>) -> impl Seed {
    SeedImpl { bytes }
}

#[cfg(test)]
pub fn from_hex(hex_str: &str) -> Result<impl Seed> {
    let bytes = hex::decode(hex_str)?;
    Ok(SeedImpl { bytes })
}
