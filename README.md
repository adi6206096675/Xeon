# Xeon Language & Toolchain (`xeon-lang`)

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-production--ready-success.svg)]()

**Xeon** is an independent, high-performance, dual-target UI programming language built entirely from scratch in Rust. It compiles custom XML-like component structures into both modern **React JavaScript (JSX)** and **system-level Rust (Wasm FFI)**.

---

## 🚀 Key Architectural Features

* **Spatial Lexer & Diagnostics:** Tracks line and column coordinates to emit Rust-style visual error pointers when syntax failures occur.
* **Multi-File Module Bundler:** Recursively traverses local file imports (e.g., `import { Card } from "./Card";`), resolves dependencies, and unifies them into a single AST.
* **Native Control Flow:** Natively parses logical control structures like `<If condition="{...}">` and `<For as="..." each="{...}">` and compiles them into conditional ternaries or mapped arrays.
* **Dual-Target Code Generation:** Transpiles application code seamlessly into React JSX or optimized Rust Wasm FFI closures.
* **The `xeonc` Toolchain:** Complete workspace management featuring project scaffolding (`xeonc init`), automated builds (`xeonc build`), and real-time multithreaded file-watching (`xeonc dev`).

---

## 📦 Installation & Setup

Ensure you have Rust installed, then clone the repository and install the compiler globally via Cargo:

```bash
cargo install --path crates/xeonc

Toolchain CommandsCommandDescriptionxeonc initScaffolds a new Xeon project (xeon.toml, src/App.xeon, and documentation).xeonc buildCompiles your multi-file workspace into the dist/ directory.xeonc devStarts a multithreaded file watcher to rebuild instantly on save (Ctrl+S).--target=wasmFlag appended to build or dev to target the custom Rust Wasm browser engine.💡 Language Syntax Overview1. Components & ImportsCode snippetimport { Button, Text, Container } from "@xeon/ui";
import { Card } from "./Card";

component App() { 
    let [theme, setTheme] = useState("dark");
    let [items, setItems] = useState(["Alpha", "Beta"]);
    let [isLoading, setIsLoading] = useState(false);
    
    return <Container layout="flex">
        <Text color="white">Welcome to Xeon</Text>
        
        <If condition="{isLoading}">
            <Text>Loading application data...</Text>
        </If>
        
        <For as="item" each="{items}">
            <Button action="{()"> console.log(item)}>{item}</Button>
        </For>
    </Container>; 
}
📂 Project StructurePlaintextxeon-lang/
├── crates/
│   ├── xeon_ast/          # Abstract Syntax Tree definitions & traits
│   ├── xeon_lexer/        # Spatial lexer and coordinate tracking
│   ├── xeon_parser/       # Recursive descent parser & diagnostic engine
│   ├── xeon_codegen_js/   # React JS & JSX code generator
│   ├── xeon_codegen_wasm/ # Rust Wasm FFI & closure code generator
│   └── xeonc/             # CLI toolchain, file watcher, and bundler
├── dist/                  # Compiled build outputs (.js and .rs)
├── src/                   # Source workspace (.xeon files)
├── XEON_DOCS.md           # Official language reference guide
└── LICENSE                # Apache 2.0 License