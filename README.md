# AArch64 Simulator
Web-based AArch64 IDE and simulator featuring microarchitectural visualizations.

Focused on exploring low-level computer architecture concepts.

## Features
* **Interactive Web IDE**:  
  Syntax highlighting, real-time error reporting, and an integrated test case editor
* **Microarchitectural visualizations**:  
  Live view of processor state including values in registers and memory, pipeline state, cache state, branch predictor 
  state
* **Modular simulation engine**:  
  Swap out architectural components (branch prediction strategy, cache policies, pipeline depth) on the fly at runtime 
  for direct comparison
* **High-performance backend**:  
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

The user interface is built with React.js and Vite, communicating directly with a high-performance simulation backend 
written in Rust and compiled to WebAssembly.

To maintain strict control over binary size and performance, the project bypasses conventional heavyweight tools like 
Emscripten and wasm-bindgen, utilizing a custom interop layer instead. The architecture is explicitly modular, allowing 
microarchitectural components to be swapped dynamically at runtime.

## Building

The React application is managed via standard Vite.js workflows. To run the frontend locally:

```bash
yarn install
yarn dev
```

For instructions on building the Rust/WASM simulation backend, refer to `simulator/readme.md`. Note that building the 
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
  Dynamic memory allocation is minimal: components are instantiated once when the simulator is configured and never 
  freed during execution. A custom bump allocator was chosen for its simplicity, minimal metadata overhead, and speed

## Challenges

- Designing and implementing a custom interop layer bridging Rust and TypeScript
- Integrating a custom bump allocator safely alongside Rust's strict borrow checker and lifetime rules
- Designing flexible, interchangeable trait-based component models
- Parsing DWARF debugging data to dynamically generate TypeScript-side documentation and ensure safe memory layout 
  interactions
- Building custom safety abstractions to govern low-level memory management within the WASM boundary