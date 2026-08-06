# Build Requirements

Required:

- Rust and `cargo` (`nightly` build, `wasm32-unknown-unknown` target)
- [`wabt`](https://github.com/webassembly/wabt) and [`binaryen`](https://github.com/WebAssembly/binaryen) (for
  WebAssembly-specific optimization passes). Specifically, `wasm-opt` from 
 `binaryen` and `wasm-strip` from `wabt` should be in PATH
- Either `bash` or `python` (used by build scripts)

Optional:

- `python` to regenerate TypeScript definitions. This is highly recommended for
  reasons described below
- [`llvm-dwarfdump`](https://github.com/llvm) enables generation of more
  detailed TypeScript type information. This is also highly recommended for the same reason.
- The [`cargo-make`](https://github.com/sagiegurari/cargo-make) crate to run the full
  build with one command

# Building

It is never required to rebuild the project to use this project since the
generated artifacts are checked into the repository. If you wish to build the
code manually, you must consider the possibility that type layouts may change
and invalidate JS-side interop code.

## Using Cargo Make

If Cargo Make is installed, run:
`cargo make`

## Without Cargo Make

If Cargo Make is not available, perform the build steps manually:

#### 1. Compile the Rust crate to WebAssembly

Build for the `wasm32-unknown-unknown` target using either the `dev` profile or
the custom `wasm` profile:

```bash
cargo build --no-default-features --target wasm32-unknown-unknown --profile wasm
```

#### 2. Package and optimize the generated WebAssembly

```bash
bash ./scripts/pkg.sh
```
or
```bash
python ./scripts/pkg.py
```

This runs WebAssembly-specific optimization passes provided by WABT and copies
the generated artifacts into the directory structure expected by Vite

#### 3. Regenerate TypeScript definitions (optional, recommended)

It is highly recommended to regenerate the TypeScript bindings whenever Rust
code is recompiled. This is because generated TypeScript definitions expose
type *layout* information that aids greatly in checking the correctness of
JS-side interop code that needs to access raw WebAssembly memory.

```bash
python ./scripts/ts-defs.py
```

## Testing

Run all unit tests on the host target:

```bash
cargo test
```

Tests should be executed natively rather than through the WebAssembly target.
