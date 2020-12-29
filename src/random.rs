use anyhow::Result;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand::{thread_rng, Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Core;
use rand_hc::Hc128Core;
use rand_jitter::JitterRng;
use rdrand::{RdRand, RdSeed};
use zeroize::Zeroize;

pub struct CompositeRng<Rng1: RngCore, Rng2: RngCore> {
    rng1: Rng1,
    rng2: Rng2,
}

impl<Rng1: RngCore, Rng2: RngCore> CompositeRng<Rng1, Rng2> {
    pub fn new(rng1: Rng1, rng2: Rng2) -> Self {
        Self { rng1, rng2 }
    }
}

impl<Rng1: RngCore, Rng2: RngCore> RngCore for CompositeRng<Rng1, Rng2> {
    fn next_u32(&mut self) -> u32 {
        self.rng1.next_u32() ^ self.rng2.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng1.next_u64() ^ self.rng2.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng1.fill_bytes(dest);
        let mut buffer = vec![0; dest.len()];
        self.rng2.fill_bytes(&mut buffer);
        for i in 0..dest.len() {
            dest[i] ^= buffer[i];
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng1.try_fill_bytes(dest)?;
        let mut buffer = vec![0; dest.len()];
        self.rng2.try_fill_bytes(&mut buffer)?;
        for i in 0..dest.len() {
            dest[i] ^= buffer[i];
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! composite_rng {
    ($rng1:expr, $rng2:expr) => {
        crate::random::CompositeRng::new($rng1, $rng2)
    };
    ($rng1:expr, $rng2:expr, $($tail:expr),+) => {
        crate::random::CompositeRng::new($rng1, composite_rng!($rng2, $($tail),+))
    };
}

pub fn secure_rng() -> Result<impl Rng> {
    // XOR together a couple different random generators.
    // This is not strictly necessary since most of those generators
    // should be secure by itself, but xoring it with others never hurts
    // for additional security. XORing a good random generator with
    // a bad one yields a random generator that is at least as good
    // as the good one.
    // This approach of xoring together hugely increases the demand on
    // hardware entropy (all of those random generators have to be seeded)
    // but that's ok because we don't use this generator a lot.

    const RESEED_THRESHOLD: u64 = 1024 * 32;

    let rdseed = rdseed_or_zeroes();
    let rdrand = rdrand_or_zeroes();
    let jitter = jitter_rng();
    let chacha = ReseedingRng::new(ChaCha20Core::from_rng(OsRng)?, RESEED_THRESHOLD, OsRng);
    let hc = ReseedingRng::new(Hc128Core::from_rng(OsRng)?, RESEED_THRESHOLD, OsRng);
    let thread = thread_rng();

    Ok(composite_rng!(
        OsRng, rdseed, rdrand, jitter, chacha, hc, thread
    ))
}

// RngOrZeroes is a random generator that either generates random values
// based on the underlying Some(rng), or - if the underlying generator
// is None, produces a series of zeroes.
// This is used so we're able to build composites with random generators
// that aren't available on all platforms. This is secure as long as it
// is in a composite with other non-zero random generators.
struct RngOrZeroes<R: RngCore>(Option<R>);
impl<R: RngCore> RngCore for RngOrZeroes<R> {
    fn next_u32(&mut self) -> u32 {
        if let Some(rng) = &mut self.0 {
            rng.next_u32()
        } else {
            0
        }
    }

    fn next_u64(&mut self) -> u64 {
        if let Some(rng) = &mut self.0 {
            rng.next_u64()
        } else {
            0
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Some(rng) = &mut self.0 {
            rng.fill_bytes(dest)
        } else {
            Zeroize::zeroize(dest);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        if let Some(rng) = &mut self.0 {
            rng.try_fill_bytes(dest)
        } else {
            Zeroize::zeroize(dest);
            Ok(())
        }
    }
}

// RandCore5Wrapper is a random generator that wraps a random generator from
// rand_core version 5 and implements the random generator from rand_core version 8.
// This is required because the rdrand crate uses rand_core 5 but we use rand_core 8.
struct RandCore5Wrapper<R: rand_core_5::RngCore>(R);
impl<R: rand_core_5::RngCore> RngCore for RandCore5Wrapper<R> {
    fn next_u32(&mut self) -> u32 {
        rand_core_5::RngCore::next_u32(&mut self.0)
    }

    fn next_u64(&mut self) -> u64 {
        rand_core_5::RngCore::next_u64(&mut self.0)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core_5::RngCore::fill_bytes(&mut self.0, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        rand_core_5::RngCore::try_fill_bytes(&mut self.0, dest)
            .map_err(|err| rand::Error::new(err.take_inner()))
    }
}

// rdseed_or_zeroes returns a random generator based on RDSEED if that instruction
// is available. Otherwise, it just outputs zeroes. This is secure because
// we only use it in an xor composite with other random generators.
fn rdseed_or_zeroes() -> impl RngCore {
    match RdSeed::new() {
        Ok(rdseed) => RngOrZeroes(Some(RandCore5Wrapper(rdseed))),
        Err(err) => {
            println!("Warning: Not able to use RDSEED random generator. Generated keys might be less random. Error message: {}", err);
            RngOrZeroes(None)
        }
    }
}

// rdrand_or_zeroes returns a random generator based on RDSEED if that instruction
// is available. Otherwise, it just outputs zeroes. This is secure because
// we only use it in an xor composite with other random generators.
fn rdrand_or_zeroes() -> impl RngCore {
    match RdRand::new() {
        Ok(rdrand) => RngOrZeroes(Some(RandCore5Wrapper(rdrand))),
        Err(err) => {
            println!("Warning: Not able to use RDRAND random generator. Generated keys might be less random. Error message: {}", err);
            RngOrZeroes(None)
        }
    }
}

fn jitter_rng() -> impl RngCore {
    fn get_nstime() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64
    }

    RandCore5Wrapper(JitterRng::new_with_timer(get_nstime))
}
