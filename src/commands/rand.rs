use clap::{Args, ValueEnum};

const BASE62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const HEX: &[u8; 16] = b"0123456789abcdef";
const DEFAULT_LENGTH: usize = 32;

/// Kind of random value to generate
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum RandOutput {
    /// Base62 string (0-9, A-Z, a-z)
    #[default]
    Base62,
    /// Hexadecimal string
    Hex,
    /// Integer with `--length` digits
    Int,
    /// Floating-point value in [0, 1) with `--length` decimal places
    Float,
}

#[derive(Args)]
pub struct RandArgs {
    /// Output format
    #[arg(long, short = 'o', value_enum, default_value_t = RandOutput::Base62)]
    pub output: RandOutput,

    /// Length of the result (characters for strings, digits for int, decimals for float)
    #[arg(long, short = 'l', default_value_t = DEFAULT_LENGTH)]
    pub length: usize,
}

pub fn run(args: RandArgs) {
    if args.length == 0 {
        eprintln!("error: --length must be greater than 0");
        std::process::exit(2);
    }

    let mut rng = Rng::new();
    let value = match args.output {
        RandOutput::Base62 => random_charset(&mut rng, BASE62, args.length),
        RandOutput::Hex => random_charset(&mut rng, HEX, args.length),
        RandOutput::Int => random_int(&mut rng, args.length),
        RandOutput::Float => random_float(&mut rng, args.length),
    };
    println!("{value}");
}

fn random_charset(rng: &mut Rng, alphabet: &[u8], length: usize) -> String {
    let n = alphabet.len() as u64;
    let mut out = String::with_capacity(length);
    for _ in 0..length {
        let idx = rng.next_u64() % n;
        out.push(alphabet[idx as usize] as char);
    }
    out
}

/// Digit string of exact `length` (no leading zero unless length is 1).
fn random_int(rng: &mut Rng, length: usize) -> String {
    let mut out = String::with_capacity(length);
    if length == 1 {
        out.push(char::from(b'0' + (rng.next_u64() % 10) as u8));
        return out;
    }
    out.push(char::from(b'1' + (rng.next_u64() % 9) as u8)); // first digit 1-9
    for _ in 1..length {
        out.push(char::from(b'0' + (rng.next_u64() % 10) as u8));
    }
    out
}

fn random_float(rng: &mut Rng, decimals: usize) -> String {
    // 53 bits of mantissa → [0, 1)
    let unit = (rng.next_u64() >> 11) as f64 / ((1u64 << 53) as f64);
    format!("{unit:.decimals$}")
}

/// Small xorshift/splitmix PRNG (no OS RNG / linker deps).
struct Rng {
    state: u64,
}

impl Rng {
    fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        seed ^= std::ptr::from_ref(&seed) as u64;
        seed ^= std::process::id() as u64;
        if seed == 0 {
            seed = 0xDEAD_BEEF_CAFE_BABE;
        }
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        // splitmix64
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}
