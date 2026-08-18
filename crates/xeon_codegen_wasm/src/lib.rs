use xeon_ast::{AstNode, Statement, UiNode, AttributeValue, XeonGenerator};

pub struct WasmEngineGenerator;

impl WasmEngineGenerator {
    fn generate_ui_tree(&self, node: &UiNode, parent_id: Option<usize>, id_counter: &mut usize) -> String {
        let current_id = *id_counter;
        *id_counter += 1;
        
        let mut code = match node {
            UiNode::Expression(expr) => {
                format!("let node_{} = xeon_sys::create_text_node(&{});\n", current_id, expr)
            },
            
            UiNode::If { condition, children } => {
                let mut block = format!("if {} {{\n", condition);
                for child in children {
                    block.push_str(&self.generate_ui_tree(child, parent_id, id_counter));
                }
                block.push_str("}\n");
                return block; 
            },
            
            UiNode::For { item_name, collection, children } => {
                let mut block = format!("for {} in {} {{\n", item_name, collection);
                for child in children {
                    block.push_str(&self.generate_ui_tree(child, parent_id, id_counter));
                }
                block.push_str("}\n");
                return block; 
            },

            UiNode::Element { tag, attributes, children } => {
                let mut el_code = format!("let node_{} = xeon_sys::create_node(\"{}\");\n", current_id, tag);
                for (key, val) in attributes {
                    match val {
                        AttributeValue::String(s) => {
                            el_code.push_str(&format!("xeon_sys::set_attribute(node_{}, \"{}\", \"{}\");\n", current_id, key, s))
                        },
                        AttributeValue::Expression(e) => {
                            let rust_closure = if e.contains("=>") {
                                let parts: Vec<&str> = e.splitn(2, "=>").collect();
                                let params = parts[0].trim();
                                let body = parts[1].trim();
                                let rust_params = if params == "()" { 
                                    "||".to_string() 
                                } else { 
                                    format!("|{}|", params.trim_matches(|c| c == '(' || c == ')')) 
                                };
                                format!("{} {{ {}; }}", rust_params, body)
                            } else { 
                                format!("|| {{ {} }}", e) 
                            };
                            el_code.push_str(&format!("xeon_sys::bind_event(node_{}, \"{}\", Box::new({}));\n", current_id, key, rust_closure));
                        }
                    }
                }
                for child in children { 
                    el_code.push_str(&self.generate_ui_tree(child, Some(current_id), id_counter)); 
                }
                el_code
            }
        };

        if let Some(parent) = parent_id { 
            code.push_str(&format!("xeon_sys::append_child(node_{}, node_{});\n", parent, current_id)); 
        }
        
        code
    }
}

impl XeonGenerator for WasmEngineGenerator {
    fn generate(&self, ast: &AstNode) -> Result<String, String> {
        match ast {
            AstNode::Program { imports, components } => {
                let mut output = String::from("// Target: Custom Rust Browser Engine (Wasm FFI)\n");
                for imp in imports {
                    output.push_str(&format!("// Wasm Module Loaded: {} from \"{}\"\n", imp.items.join(", "), imp.module));
                }

                for comp in components {
                    output.push_str(&format!("\n// Component: {}\n", comp.name));
                    for stmt in &comp.body {
                        if let Statement::StateDeclaration { state_name, setter_name, initial_value } = stmt {
                            output.push_str(&format!("xeon_sys::allocate_state(\"{}\", \"{}\", {});\n", state_name, setter_name, initial_value));
                        }
                    }
                    let mut id_counter = 0;
                    output.push_str(&self.generate_ui_tree(&comp.return_tree, None, &mut id_counter));
                }
                Ok(output)
            }
        }
    }
}