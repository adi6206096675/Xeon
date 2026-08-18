use xeon_ast::{AstNode, Statement, UiNode, AttributeValue, XeonGenerator};

pub struct ReactJsGenerator;

impl ReactJsGenerator {
    fn generate_ui_tree(&self, node: &UiNode, indent: usize) -> String {
        let spaces = " ".repeat(indent);
        match node {
            UiNode::Expression(expr) => format!("{}{{{}}}", spaces, expr),
            
            UiNode::If { condition, children } => {
                let mut tree = format!("{}{{\n{} ? (\n{}<>\n", spaces, condition, spaces);
                for child in children {
                    tree.push_str(&self.generate_ui_tree(child, indent + 2));
                    tree.push('\n');
                }
                tree.push_str(&format!("{}</>\n{}) : null\n{}}}", spaces, spaces, spaces));
                tree
            },
            
            UiNode::For { item_name, collection, children } => {
                let mut tree = format!("{}{{\n{}.map(({}) => (\n{}<>\n", spaces, collection, item_name, spaces);
                for child in children {
                    tree.push_str(&self.generate_ui_tree(child, indent + 2));
                    tree.push('\n');
                }
                tree.push_str(&format!("{}</>\n{}))\n{}}}", spaces, spaces, spaces));
                tree
            },

            UiNode::Element { tag, attributes, children } => {
                let mut attrs = String::new();
                for (key, val) in attributes { 
                    match val {
                        AttributeValue::String(s) => attrs.push_str(&format!(" {}=\"{}\"", key, s)),
                        AttributeValue::Expression(e) => attrs.push_str(&format!(" {}={{{}}}", key, e)),
                    }
                }

                if children.is_empty() {
                    format!("{}<{}{} />", spaces, tag, attrs)
                } else {
                    let mut tree = format!("{}<{}{}>\n", spaces, tag, attrs);
                    for child in children {
                        tree.push_str(&self.generate_ui_tree(child, indent + 2));
                        tree.push('\n');
                    }
                    tree.push_str(&format!("{}</{}>", spaces, tag));
                    tree
                }
            }
        }
    }
}

impl XeonGenerator for ReactJsGenerator {
    fn generate(&self, ast: &AstNode) -> Result<String, String> {
        match ast {
            AstNode::Program { imports, components } => {
                let mut output = String::from("import { useState } from 'react';\n");
                for imp in imports {
                    output.push_str(&format!("import {{ {} }} from \"{}\";\n", imp.items.join(", "), imp.module));
                }
                output.push('\n');

                for comp in components {
                    output.push_str(&format!("export default function {}() {{\n", comp.name));
                    for stmt in &comp.body {
                        if let Statement::StateDeclaration { state_name, setter_name, initial_value } = stmt {
                            output.push_str(&format!("  const [{}, {}] = useState({});\n", state_name, setter_name, initial_value));
                        }
                    }
                    let ui_code = self.generate_ui_tree(&comp.return_tree, 2);
                    output.push_str(&format!("\n  return (\n{}\n  );\n}}\n", ui_code));
                }
                Ok(output)
            }
        }
    }
}