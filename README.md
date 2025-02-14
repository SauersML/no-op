# no‑op

`no‑op` is a minimal Rust binary that literally does nothing.

## Features

- **Ultra‑Minimal Binary:** Contains only a `main()` function that does nothing.
- **Optimized for Speed:** Uses Cargo release settings (e.g., `panic = "abort"`, `opt-level = "z"`, LTO, and a single codegen unit) to produce a fast‑startup executable.
- **No Dependencies:** A self‑contained example without external dependencies.

## Usage

### Building

To build the binary in release mode, run:

```bash
cargo build --release
```

This will create an optimized executable in the `target/release` directory.

### Running

Since the program does nothing, running it will immediately exit with status 0:

```bash
./target/release/no-op
```

### Benchmarks
See benchmarks (after building) with:
```
cargo test --release -- --nocapture
```
