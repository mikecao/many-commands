# mc

A CLI that runs many commands. One binary, many subcommands:

```text
mc <command> [options]
```

Built in Rust for cross-platform distribution (Windows, macOS, Linux).

## Requirements

- [Rust](https://rustup.rs/) (stable)

On Windows, either:

- **MSVC** — Visual Studio Build Tools with the “Desktop development with C++” workload, or
- **GNU** — `rustup toolchain install stable-gnu` (this repo may use a directory override for `stable-gnu`)

## Build

```bash
# debug
cargo build

# release (stripped + LTO)
cargo build --release
```

Binaries:

| Profile | Path |
|---|---|
| Debug | `target/debug/mc` (`mc.exe` on Windows) |
| Release | `target/release/mc` |

Run without installing:

```bash
cargo run -- <command> [args...]
```

## Commands

### `ts` — Unix timestamp

Print the current Unix time.

```bash
mc ts          # seconds
mc ts --ms     # milliseconds
```

### `uuid` — Random UUID

Print a random UUID.

```bash
mc uuid                 # version 4 (default)
mc uuid --version 4
mc uuid --version 7     # time-ordered
mc uuid -v 7
```

| Option | Default | Description |
|---|---|---|
| `-v`, `--version` | `4` | UUID version: `4` (random) or `7` (time-ordered). Aliases: `v4`, `v7` |

App version is still `mc --version` / `mc -V` (not the uuid subcommand).

### `hash` — Hash a string

Hash input with a chosen algorithm and digest encoding.

```bash
mc hash [OPTIONS] [INPUT]
```

| Option | Default | Description |
|---|---|---|
| `-n`, `--name` | `sha256` | Hash algorithm |
| `-d`, `--digest` | `hex` | Output encoding: `hex`, `base64`, `base64url` |
| `INPUT` | *(random 32 bytes)* | String to hash; if omitted, a random value is hashed |

Examples:

```bash
mc hash hello
mc hash --name=sha256 --digest=base64 hello
mc hash -n md5 -d base64url "some text"
mc hash                          # hash random input
```

#### Algorithm commands

Each algorithm is also available as a top-level command:

```bash
mc sha256 hello
mc md5 --digest=base64 hello
mc sha3-512
```

#### Supported algorithms

| Group | Names |
|---|---|
| SHA-2 | `sha224`, `sha256`, `sha384`, `sha512`, `sha512-224`, `sha512-256` |
| SHA-3 | `sha3-224`, `sha3-256`, `sha3-384`, `sha3-512` |
| SHAKE | `shake128` (32-byte output), `shake256` (64-byte output) |
| BLAKE | `blake2b512`, `blake2s256` |
| Legacy | `md5`, `md4`, `md2`, `sha1`, `ripemd160` |
| Other | `sm3`, `whirlpool` |

> **Note:** Legacy algorithms (MD\*, SHA-1) are provided for compatibility only. Prefer SHA-2/SHA-3/BLAKE for security-sensitive use.

## Project layout

```text
src/
  main.rs              # clap root + subcommand dispatch
  commands/
    mod.rs
    ts.rs              # timestamp command
    hash.rs            # hash command + algorithm logic
    uuid.rs            # UUID generator
```


### Adding a command

1. Create `src/commands/foo.rs` with an args struct and a `run` function.
2. Register it in `src/commands/mod.rs` (`pub mod foo;`).
3. Add a variant to the `Commands` enum in `src/main.rs` and handle it in `main`.

## License

Proprietary / unlicensed unless otherwise stated.
