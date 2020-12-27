#[cfg(test)]
use anyhow::Result;

pub struct Seed {
    seed: Vec<u8>,
}

impl Seed {
    pub fn from_bytes(seed: Vec<u8>) -> Self {
        Self { seed }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.seed
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.seed
    }

    #[cfg(test)]
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let seed = hex::decode(hex_str)?;
        Ok(Self { seed })
    }
}
