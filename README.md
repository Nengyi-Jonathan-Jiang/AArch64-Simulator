# AArch64 Simulator
Web-based AArch64 IDE and simulator featuring microarchitectural visualizations.

Focused on exploring low-level computer architecture concepts.

The current application provides syntax highlighting, real-time diagnostics, test-case editing, single-step execution, 
and register and memory inspection. A modular Rust/WebAssembly backend is under development to model branch prediction, 
caching, and pipelining.

## Features (In Development)
* **Interactive Web IDE**:  
  Syntax highlighting, real-time error reporting, and an integrated test case editor
* **Microarchitectural visualizations**:  
  Live view of processor state including values in registers and memory, pipeline state, cache state, branch predictor 
  state
* **Modular simulation engine**:  
  Reconfigurable architectural components (branch prediction strategy, cache policies, pipeline depth) at runtime 
* **Rust/WebAssembly backend**:  
  Rust-powered core compiled to WebAssembly (WASM) for better, more stable performance

## Progress
- [ ] UI elements
  - [x] Syntax highlighting  
  - [x] Error messages
  - [ ] Breakpoints and other debugging features
  - [x] Test case editor
  - [ ] Visualizations
    - [x] Registers/Memory
    - [ ] Cache _(pending full simulation)_
    - [ ] Pipeline _(pending full simulation)_
    - [ ] Branch predictor _(pending full simulation)_
- [ ] Assembly simulation
  - [x] Temporary basic simulation in TypeScript
  - [ ] Full simulation with all features in Rust/WASM
    - [x] Branch predictor implementations
    - [ ] Cache implementations
    - [ ] Pipeline implementations

## Architecture

The user interface is built with React, TypeScript, and Vite. The current application uses a basic TypeScript simulation 
path while a more detailed Rust/WebAssembly backend is being developed for more detailed microarchitectural modeling.

The architecture of the simulation backend is explicitly modular, allowing microarchitectural components to be swapped 
dynamically at runtime.

To maintain strict control over binary size and performance, the project bypasses conventional tools like Emscripten and
wasm-bindgen, utilizing a custom interop layer instead.

## Building

The React application is managed via standard Vite.js workflows. To run the frontend locally:

```bash
yarn install
yarn dev
```

For instructions on building the Rust/WASM simulation backend, refer to [`simulator/README.md`](simulator/README.md). Note that building the 
backend from source is **not** necessary unless you wish to edit the Rust code, as all required artifacts are already 
checked into the repository. 

## Design Decisions

- **Rust for Core Simulation**:   
  Performance is critical. Rust offers a rich type system that naturally models hardware components while avoiding the 
  runtime overhead, garbage collection, and dynamic object allocation penalties typical of JavaScript/TypeScript
- **Minimal Dependencies**:   
  To significantly shrink the final WebAssembly binary size, the simulator avoids standard interop libraries and runtime
  bloat, implementing only what is necessary. Metaprogramming libraries, however, are utilized where appropriate to 
  facilitate safer code abstractions
- **Custom Bump Allocator**: 
  Dynamic memory allocation is minimal: simulator components are allocated together and are not freed individually.  
  Thus, a custom bump allocator was chosen for its simplicity, minimal metadata overhead, and speed. When the simulator 
  is reconfigured, the arena is reset and all components are reconstructed

## Challenges

- Designing and implementing a custom interop layer bridging Rust and TypeScript
- Integrating a custom bump allocator safely alongside Rust's strict borrow checker and lifetime rules
- Designing flexible, interchangeable trait-based component models
- Parsing DWARF debug information to generate layout-aware TypeScript bindings and reduce ABI mismatches when accessing 
  WebAssembly memory
- Building custom safety abstractions to govern low-level memory management within the WASM boundary