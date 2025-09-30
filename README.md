# Floraison

**Procedural 3D flower generation based on botanical principles**

Floraison is a web-based tool for generating botanically accurate 3D flowers using pure algorithmic approaches. Based on the SIGGRAPH 2005 paper by Ijiri et al., it uses floral diagrams and inflorescence patterns to create diverse flower models without manual editing.

## Features

- **Botanically structured**: Uses floral diagrams (pistil, stamen, petal, sepal arrangements) and inflorescence patterns (raceme, umbel, dichasium, etc.)
- **Purely procedural**: No manual mesh editing—all geometry generated from parameters
- **High-performance compute**: Rust core compiled to WebAssembly
- **Interactive preview**: Real-time 3D visualization with Three.js
- **glTF export**: Industry-standard format for use in other tools

## Tech Stack

- **Compute**: Rust → WebAssembly (wasm-bindgen)
- **Frontend**: SvelteKit + TailwindCSS
- **Rendering**: Three.js
- **Export**: glTF 2.0

## Project Status

🚧 In active development for the [Made with Claude](https://build.anthropic.com) contest

## Documentation

- [Technical Overview](docs/TECHNICAL_OVERVIEW.md) - Architecture and implementation details
- [Implementation Roadmap](docs/ROADMAP.md) - Development plan and progress

## Reference

Based on: *Floral diagrams and inflorescences: Interactive flower modeling using botanical structural constraints*
Ijiri, Owada, Okabe, Igarashi (SIGGRAPH 2005)

## License

MIT
