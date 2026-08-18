use std::env;
use std::fs;
use std::path::Path;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::sync::mpsc::channel;

use xeon_lexer::Lexer;
use xeon_parser::{Parser, ParseError};
use xeon_ast::{AstNode, XeonGenerator, ComponentNode, ImportNode};
use xeon_codegen_js::ReactJsGenerator;
use xeon_codegen_wasm::WasmEngineGenerator;

fn print_diagnostic(file_path: &str, source: &str, err: &ParseError) {
    eprintln!("\n❌ error: {}", err.message);
    eprintln!("  --> {}:{}:{}", file_path, err.line, err.col);
    let lines: Vec<&str> = source.lines().collect();
    if err.line > 0 && err.line <= lines.len() {
        let code_line = lines[err.line - 1];
        eprintln!("   |");
        eprintln!("{:>3} | {}", err.line, code_line);
        eprintln!("   | {}^{}", " ".repeat(err.col.saturating_sub(1)), "-".repeat(5));
    }
    eprintln!();
}

fn resolve_and_parse(file_path: &Path, parsed_components: &mut Vec<ComponentNode>, resolved_imports: &mut Vec<ImportNode>) -> Result<(), ()> {
    let source_code = fs::read_to_string(file_path).map_err(|_| eprintln!("❌ Failed to resolve module: {}", file_path.display()))?;
    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(AstNode::Program { imports, mut components }) => {
            parsed_components.append(&mut components);
            for import in imports {
                if import.module.starts_with("./") || import.module.starts_with("../") {
                    let base_dir = file_path.parent().unwrap_or(Path::new(""));
                    let mut dep_path = base_dir.join(&import.module);
                    dep_path.set_extension("xeon");
                    resolve_and_parse(&dep_path, parsed_components, resolved_imports)?;
                } else {
                    resolved_imports.push(import);
                }
            }
            Ok(())
        },
        Err(err) => {
            print_diagnostic(file_path.to_str().unwrap(), &source_code, &err);
            Err(())
        }
    }
}

fn compile_project(target: &str) {
    println!("🔄 Building Workspace...");
    let entry_path = "src/App.xeon"; // Determined by xeon.toml in a full release
    
    let mut all_components = Vec::new();
    let mut all_imports = Vec::new();

    if resolve_and_parse(Path::new(entry_path), &mut all_components, &mut all_imports).is_ok() {
        let bundled_ast = AstNode::Program { imports: all_imports, components: all_components };
        let generator = if target == "wasm" { WasmEngineGenerator.generate(&bundled_ast).unwrap() } 
                        else { ReactJsGenerator.generate(&bundled_ast).unwrap() };

        fs::create_dir_all("dist").expect("Failed to create dist directory");
        let output_ext = if target == "wasm" { "rs" } else { "js" };
        let output_path = format!("dist/bundle_App.{}", output_ext);
        
        fs::write(&output_path, generator).expect("Failed to write output file");
        println!("✅ Build complete -> {}", output_path);
    }
}

fn init_project() {
    println!("📦 Initializing new Xeon workspace...");
    fs::create_dir_all("src").unwrap();
    
    let toml_config = r#"[package]
name = "xeon-app"
version = "0.1.0"
target = "js" # Change to "wasm" for custom engine bindings

[dependencies]
"@xeon/ui" = "latest"
"#;
    
    let initial_app = r#"import { Button, Text, Container } from "@xeon/ui";

component App() { 
    let [data, setData] = useState(["Alpha", "Beta"]);
    let [isLoading, setIsLoading] = useState(false);
    
    return <Container layout="flex">
        <Text color="white">Xeon Production Workspace</Text>
        
        <If condition="{isLoading}">
            <Text>Loading data...</Text>
        </If>
        
        <For as="item" each="{data}">
            <Button action={() => console.log(item)}>{item}</Button>
        </For>
    </Container>; 
}
"#;

    fs::write("xeon.toml", toml_config).unwrap();
    fs::write("src/App.xeon", initial_app).unwrap();
    println!("🎉 Workspace created! Run `xeonc dev` to start programming.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");
    
    let target = if args.contains(&"--target=wasm".to_string()) { "wasm" } else { "js" };

    println!("🚀 xeonc v0.1.0 - The Xeon Toolchain");

    match command {
        "init" => init_project(),
        "build" => compile_project(target),
        "dev" => {
            compile_project(target);
            println!("👀 Watching project for changes...");
            let (tx, rx) = channel();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
            watcher.watch(Path::new("src"), RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(Ok(Event { kind: EventKind::Modify(_), .. })) => compile_project(target),
                    _ => {} 
                }
            }
        },
        _ => {
            println!("Usage:");
            println!("  xeonc init          Initialize a new project (xeon.toml & src/)");
            println!("  xeonc build         Compile project to dist/");
            println!("  xeonc dev           Compile and watch for changes");
            println!("  --target=wasm       Optional flag to compile for Rust Browser Engine");
        }
    }
}