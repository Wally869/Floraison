# Epic 1: Project Setup & Infrastructure

**Phase**: 1 - Foundation & Single Flower MVP
**Goal**: Initialize all necessary project structure, tooling, and build configuration.
**Estimated Effort**: 4-6 hours

---

## Task 1.1: Initialize Rust Workspace ✅

**Description**: Set up Cargo workspace with multiple crates for separation of concerns.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Workspace created with `Cargo.toml` at root
- [x] Crate structure created:
  - `floraison-core` (library)
  - `floraison-components` (library)
  - `floraison-wasm` (cdylib + rlib)
- [x] All crates compile successfully
- [x] Basic `lib.rs` files with placeholder modules
- [x] Workspace dependencies configured (glam, serde, wasm-bindgen, etc.)
- [x] Release profile optimized for WASM size

**Dependencies**: None

**Technical Notes**:
- Use workspace dependencies for shared crates (glam, serde)
- Set up workspace-level profile configurations

**Effort**: 1 hour

---

## Task 1.2: Configure WASM Build System ✅

**Description**: Set up wasm-pack configuration and build scripts.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] `wasm-pack` installed (v0.13.1)
- [x] `Cargo.toml` for `floraison-wasm` configured with:
  - `crate-type = ["cdylib", "rlib"]`
  - `wasm-bindgen` dependency
  - Proper feature flags
- [x] Build scripts created:
  - `build-wasm.sh` (Bash)
  - `build-wasm.ps1` (PowerShell)
  - npm scripts: `wasm:build`, `wasm:dev`
- [x] Generated WASM outputs to `floraison-ui/src/lib/wasm/`
- [x] WASM loader utility created (`loader.ts`)
- [x] Vite config updated for WASM support
- [x] Import tests pass (3/3)
- [x] Documentation created (README.md, TESTING.md)

**Dependencies**: Task 1.1

**Technical Notes**:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

Build command: `wasm-pack build --target web --out-dir ../frontend/src/lib/wasm`

**Effort**: 2 hours

---

## Task 1.3: Initialize SvelteKit Frontend ✅

**Description**: Create SvelteKit project with TypeScript and TailwindCSS.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] SvelteKit project initialized in `floraison-ui/` directory
- [x] TypeScript configured
- [x] TailwindCSS v4 installed and configured
- [x] Dev server runs successfully (`npm run dev`)
- [x] Testing setup (Vitest + Playwright)
- [x] Vite config updated to handle WASM imports (completed in Task 1.2)

**Dependencies**: None

**Technical Notes**:
```bash
npm create svelte@latest frontend
cd frontend
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
npm install three @types/three
```

Vite config for WASM:
```js
export default defineConfig({
  server: {
    fs: {
      allow: ['..']
    }
  }
});
```

**Effort**: 1.5 hours

---

## Task 1.4: Set Up Development Workflow ✅

**Description**: Configure hot reload, build scripts, and development conveniences.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Root-level `package.json` with unified commands
- [x] `npm run dev`: Watches Rust (cargo-watch), rebuilds WASM, runs SvelteKit dev server (concurrently)
- [x] `npm run build`: Release build for both Rust and frontend
- [x] `.gitignore` configured properly for both Rust and Node
- [x] README instructions for running locally
- [x] Additional commands: test, check, format, lint, clean
- [x] VSCode settings and extensions recommended
- [x] GitHub Actions CI workflow

**Dependencies**: Tasks 1.1, 1.2, 1.3

**Technical Notes**:
Consider using `cargo-watch` for Rust file monitoring:
```bash
cargo watch -i frontend/ -s "wasm-pack build ..."
```

Or create a simple bash/PowerShell script for Windows compatibility.

**Effort**: 1.5 hours
