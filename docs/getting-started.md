# Getting Started

## Prerequisites

- Rust toolchain (`cargo`)
- `Rscript` in `PATH` for runtime execution (`rr run`)

## Build RR

```bash
cargo build
```

Run the binary through Cargo:

```bash
cargo run -- --help
```

## First Program

Create `main.rr`:

```rr
fn main() {
  print(1 + 2);
}
main();
```

Run it:

```bash
cargo run -- run . -O1
```

## Compile to R only

```bash
cargo run -- main.rr -o main.R --no-runtime -O2
```

This writes a self-contained `.R` script with RR runtime helpers injected at the top.

## Build a directory

```bash
cargo run -- build . --out-dir build -O2
```

Behavior:

- Recursively finds `.rr` files.
- Skips `build/`, `target/`, `.git/` while scanning.
- Writes mirrored output paths under `build/` with `.R` extension.
