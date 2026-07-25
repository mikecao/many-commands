mod commands;

use clap::{Parser, Subcommand};
use commands::hash::{HashAliasArgs, HashArgs, HashName};

/// mc — a multi-command toolkit
#[derive(Parser)]
#[command(name = "mc", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the current unix timestamp
    Ts(commands::ts::TsArgs),

    /// Hash a string (or random input if omitted)
    Hash(HashArgs),

    /// Print a random UUID
    Uuid(commands::uuid::UuidArgs),

    // --- hash algorithms ---
    /// Hash with SHA-224
    #[command(name = "sha224")]
    Sha224(HashAliasArgs),
    /// Hash with SHA-256
    #[command(name = "sha256")]
    Sha256(HashAliasArgs),
    /// Hash with SHA-384
    #[command(name = "sha384")]
    Sha384(HashAliasArgs),
    /// Hash with SHA-512
    #[command(name = "sha512")]
    Sha512(HashAliasArgs),
    /// Hash with SHA-512/224
    #[command(name = "sha512-224")]
    Sha512_224(HashAliasArgs),
    /// Hash with SHA-512/256
    #[command(name = "sha512-256")]
    Sha512_256(HashAliasArgs),
    /// Hash with SHA3-224
    #[command(name = "sha3-224")]
    Sha3_224(HashAliasArgs),
    /// Hash with SHA3-256
    #[command(name = "sha3-256")]
    Sha3_256(HashAliasArgs),
    /// Hash with SHA3-384
    #[command(name = "sha3-384")]
    Sha3_384(HashAliasArgs),
    /// Hash with SHA3-512
    #[command(name = "sha3-512")]
    Sha3_512(HashAliasArgs),
    /// Hash with SHAKE128
    #[command(name = "shake128")]
    Shake128(HashAliasArgs),
    /// Hash with SHAKE256
    #[command(name = "shake256")]
    Shake256(HashAliasArgs),
    /// Hash with BLAKE2b-512
    #[command(name = "blake2b512")]
    Blake2b512(HashAliasArgs),
    /// Hash with BLAKE2s-256
    #[command(name = "blake2s256")]
    Blake2s256(HashAliasArgs),
    /// Hash with MD5
    #[command(name = "md5")]
    Md5(HashAliasArgs),
    /// Hash with MD4
    #[command(name = "md4")]
    Md4(HashAliasArgs),
    /// Hash with MD2
    #[command(name = "md2")]
    Md2(HashAliasArgs),
    /// Hash with SHA-1
    #[command(name = "sha1")]
    Sha1(HashAliasArgs),
    /// Hash with RIPEMD-160
    #[command(name = "ripemd160")]
    Ripemd160(HashAliasArgs),
    /// Hash with SM3
    #[command(name = "sm3")]
    Sm3(HashAliasArgs),
    /// Hash with Whirlpool
    #[command(name = "whirlpool")]
    Whirlpool(HashAliasArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ts(args) => commands::ts::run(args),
        Commands::Hash(args) => commands::hash::run(args),
        Commands::Uuid(args) => commands::uuid::run(args),
        Commands::Sha224(args) => commands::hash::run_alias(HashName::Sha224, args),
        Commands::Sha256(args) => commands::hash::run_alias(HashName::Sha256, args),
        Commands::Sha384(args) => commands::hash::run_alias(HashName::Sha384, args),
        Commands::Sha512(args) => commands::hash::run_alias(HashName::Sha512, args),
        Commands::Sha512_224(args) => commands::hash::run_alias(HashName::Sha512_224, args),
        Commands::Sha512_256(args) => commands::hash::run_alias(HashName::Sha512_256, args),
        Commands::Sha3_224(args) => commands::hash::run_alias(HashName::Sha3_224, args),
        Commands::Sha3_256(args) => commands::hash::run_alias(HashName::Sha3_256, args),
        Commands::Sha3_384(args) => commands::hash::run_alias(HashName::Sha3_384, args),
        Commands::Sha3_512(args) => commands::hash::run_alias(HashName::Sha3_512, args),
        Commands::Shake128(args) => commands::hash::run_alias(HashName::Shake128, args),
        Commands::Shake256(args) => commands::hash::run_alias(HashName::Shake256, args),
        Commands::Blake2b512(args) => commands::hash::run_alias(HashName::Blake2b512, args),
        Commands::Blake2s256(args) => commands::hash::run_alias(HashName::Blake2s256, args),
        Commands::Md5(args) => commands::hash::run_alias(HashName::Md5, args),
        Commands::Md4(args) => commands::hash::run_alias(HashName::Md4, args),
        Commands::Md2(args) => commands::hash::run_alias(HashName::Md2, args),
        Commands::Sha1(args) => commands::hash::run_alias(HashName::Sha1, args),
        Commands::Ripemd160(args) => commands::hash::run_alias(HashName::Ripemd160, args),
        Commands::Sm3(args) => commands::hash::run_alias(HashName::Sm3, args),
        Commands::Whirlpool(args) => commands::hash::run_alias(HashName::Whirlpool, args),
    }
}
