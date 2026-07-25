use clap::{Args, ValueEnum};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Builder;

/// UUID version to generate
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum UuidVersion {
    /// Random UUID (RFC 4122)
    #[default]
    #[value(name = "4", alias = "v4")]
    V4,
    /// Time-ordered UUID (RFC 9562)
    #[value(name = "7", alias = "v7")]
    V7,
}

#[derive(Args)]
pub struct UuidArgs {
    /// UUID version (`4` or `7`)
    #[arg(long = "version", short = 'v', value_enum, default_value_t = UuidVersion::V4)]
    pub version: UuidVersion,
}

pub fn run(args: UuidArgs) {
    let id = match args.version {
        UuidVersion::V4 => new_v4(),
        UuidVersion::V7 => new_v7(),
    };
    println!("{id}");
}

fn new_v4() -> uuid::Uuid {
    let mut bytes = [0u8; 16];
    fill_random(&mut bytes);
    Builder::from_random_bytes(bytes).into_uuid()
}

fn new_v7() -> uuid::Uuid {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the unix epoch")
        .as_millis() as u64;
    let mut rand = [0u8; 10];
    fill_random(&mut rand);
    Builder::from_unix_timestamp_millis(millis, &rand).into_uuid()
}

/// Non-cryptographic PRNG fill (avoids getrandom / system linker deps on Windows GNU).
fn fill_random(buf: &mut [u8]) {
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    seed ^= std::ptr::from_ref(&seed) as u64;
    seed ^= std::process::id() as u64;

    for chunk in buf.chunks_mut(8) {
        seed = seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        let bytes = z.to_le_bytes();
        chunk.copy_from_slice(&bytes[..chunk.len()]);
    }
}
