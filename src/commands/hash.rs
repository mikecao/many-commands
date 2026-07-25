use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use clap::{Args, ValueEnum};
use digest::{Digest, ExtendableOutput, Update};

/// Supported hash algorithms
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum HashName {
    #[value(name = "sha224")]
    Sha224,
    #[default]
    #[value(name = "sha256")]
    Sha256,
    #[value(name = "sha384")]
    Sha384,
    #[value(name = "sha512")]
    Sha512,
    #[value(name = "sha512-224")]
    Sha512_224,
    #[value(name = "sha512-256")]
    Sha512_256,
    #[value(name = "sha3-224")]
    Sha3_224,
    #[value(name = "sha3-256")]
    Sha3_256,
    #[value(name = "sha3-384")]
    Sha3_384,
    #[value(name = "sha3-512")]
    Sha3_512,
    #[value(name = "shake128")]
    Shake128,
    #[value(name = "shake256")]
    Shake256,
    #[value(name = "blake2b512")]
    Blake2b512,
    #[value(name = "blake2s256")]
    Blake2s256,
    #[value(name = "md5")]
    Md5,
    #[value(name = "md4")]
    Md4,
    #[value(name = "md2")]
    Md2,
    #[value(name = "sha1")]
    Sha1,
    #[value(name = "ripemd160")]
    Ripemd160,
    #[value(name = "sm3")]
    Sm3,
    #[value(name = "whirlpool")]
    Whirlpool,
}

/// Output encoding for the digest
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum DigestFormat {
    #[default]
    Hex,
    Base64,
    #[value(name = "base64url")]
    Base64Url,
}

#[derive(Args)]
pub struct HashArgs {
    /// Hash algorithm
    #[arg(long, short = 'n', value_enum, default_value_t = HashName::Sha256)]
    pub name: HashName,

    /// Digest output format
    #[arg(long, short = 'd', value_enum, default_value_t = DigestFormat::Hex)]
    pub digest: DigestFormat,

    /// String to hash (random bytes if omitted)
    pub input: Option<String>,
}

/// Args for algorithm alias commands (`mc sha256`, etc.)
#[derive(Args)]
pub struct HashAliasArgs {
    /// Digest output format
    #[arg(long, short = 'd', value_enum, default_value_t = DigestFormat::Hex)]
    pub digest: DigestFormat,

    /// String to hash (random bytes if omitted)
    pub input: Option<String>,
}

pub fn run(args: HashArgs) {
    execute(args.name, args.digest, args.input.as_deref());
}

pub fn run_alias(name: HashName, args: HashAliasArgs) {
    execute(name, args.digest, args.input.as_deref());
}

fn execute(name: HashName, format: DigestFormat, input: Option<&str>) {
    let data: Vec<u8> = match input {
        Some(s) => s.as_bytes().to_vec(),
        None => random_bytes(32),
    };

    let digest = hash_bytes(name, &data);
    println!("{}", encode_digest(&digest, format));
}

/// Non-cryptographic random bytes for the no-input fallback.
fn random_bytes(len: usize) -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    seed ^= std::ptr::from_ref(&seed) as u64;
    seed ^= std::process::id() as u64;

    let mut out = vec![0u8; len];
    for chunk in out.chunks_mut(8) {
        // splitmix64
        seed = seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        let bytes = z.to_le_bytes();
        chunk.copy_from_slice(&bytes[..chunk.len()]);
    }
    out
}

fn hash_bytes(name: HashName, data: &[u8]) -> Vec<u8> {
    match name {
        HashName::Sha224 => digest_fixed(sha2::Sha224::new(), data),
        HashName::Sha256 => digest_fixed(sha2::Sha256::new(), data),
        HashName::Sha384 => digest_fixed(sha2::Sha384::new(), data),
        HashName::Sha512 => digest_fixed(sha2::Sha512::new(), data),
        HashName::Sha512_224 => digest_fixed(sha2::Sha512_224::new(), data),
        HashName::Sha512_256 => digest_fixed(sha2::Sha512_256::new(), data),
        HashName::Sha3_224 => digest_fixed(sha3::Sha3_224::new(), data),
        HashName::Sha3_256 => digest_fixed(sha3::Sha3_256::new(), data),
        HashName::Sha3_384 => digest_fixed(sha3::Sha3_384::new(), data),
        HashName::Sha3_512 => digest_fixed(sha3::Sha3_512::new(), data),
        HashName::Shake128 => digest_xof(sha3::Shake128::default(), data, 32),
        HashName::Shake256 => digest_xof(sha3::Shake256::default(), data, 64),
        HashName::Blake2b512 => digest_fixed(blake2::Blake2b512::new(), data),
        HashName::Blake2s256 => digest_fixed(blake2::Blake2s256::new(), data),
        HashName::Md5 => digest_fixed(md5::Md5::new(), data),
        HashName::Md4 => digest_fixed(md4::Md4::new(), data),
        HashName::Md2 => digest_fixed(md2::Md2::new(), data),
        HashName::Sha1 => digest_fixed(sha1::Sha1::new(), data),
        HashName::Ripemd160 => digest_fixed(ripemd::Ripemd160::new(), data),
        HashName::Sm3 => digest_fixed(sm3::Sm3::new(), data),
        HashName::Whirlpool => digest_fixed(whirlpool::Whirlpool::new(), data),
    }
}

fn digest_fixed<D: Digest>(mut hasher: D, data: &[u8]) -> Vec<u8> {
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn digest_xof<X: ExtendableOutput + Update>(mut hasher: X, data: &[u8], out_len: usize) -> Vec<u8> {
    hasher.update(data);
    hasher.finalize_boxed(out_len).to_vec()
}

fn encode_digest(bytes: &[u8], format: DigestFormat) -> String {
    match format {
        DigestFormat::Hex => hex_encode(bytes),
        DigestFormat::Base64 => STANDARD.encode(bytes),
        DigestFormat::Base64Url => URL_SAFE_NO_PAD.encode(bytes),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}
