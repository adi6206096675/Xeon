# Xeon Language & Toolchain Reference (v0.1.0)

Welcome to Xeon, a dual-target UI programming language engineered for high-performance web applications and custom Rust browser engines.

---

## 🚀 CLI Toolchain Commands

| Command | Description |
| :--- | :--- |
| `xeonc init` | Scaffolds a new Xeon workspace (`xeon.toml`, `src/App.xeon`, and `XEON_DOCS.md`). |
| `xeonc build` | Compiles your multi-file project tree into optimized outputs inside `dist/`. |
| `xeonc dev` | Launches a high-performance, multithreaded file watcher to rebuild instantly on save (`Ctrl+S`). |
| `--target=wasm` | Flag appended to build/dev commands to compile targets for the custom Rust Wasm FFI engine. |

---

## 💡 Language Syntax & Features

### 1. Component Declaration
Every Xeon file defines modular UI components using the `component` keyword:
```xeon
component App() {
    return <Container layout="flex">
        <Text>Hello Xeon</Text>
    </Container>;
}
2. Module Import System
Xeon supports external NPM libraries and local relative file imports with automatic multi-file AST bundling:

Code snippet
import { Button, Text, Container } from "@xeon/ui";
import { Card } from "./Card";
3. Reactive State Hooks
Manage local component state seamlessly:

Code snippet
let [theme, setTheme] = useState("dark");
let [data, setData] = useState(["Alpha", "Beta"]);
4. Event Handlers & Closures
Interactivity is handled via inline arrow functions that automatically transpile to React callbacks or Rust system closures:

Code snippet
<Button action="{()"> setTheme("light")}>Toggle Theme</Button>
5. Native Control Flow
Xeon features built-in structural logic tags for conditionals and iteration:

Conditionals (<If>):

Code snippet
<If condition="{isLoading}">
    <Text>Loading data...</Text>
</If>
Loops (<For>):

Code snippet
<For as="item" each="{data}">
    <Button action="{()"> console.log(item)}>{item}</Button>
</For>
🛠️ Production Diagnostics
Xeon features a spatial lexer and diagnostic engine. If a syntax mistake occurs (such as a mismatched closing tag like </Containers>), the compiler halts execution and outputs a Rust-style visual error pointer detailing the exact file name, line number, and column coordinate.