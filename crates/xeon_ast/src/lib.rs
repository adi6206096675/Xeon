#[derive(Debug, Clone, PartialEq)]
pub enum XeonType { String, Number, Boolean, Custom(String) }

#[derive(Debug, Clone)]
pub struct Prop {
    pub name: String,
    pub prop_type: XeonType,
}

#[derive(Debug, Clone)]
pub enum Statement {
    StateDeclaration {
        state_name: String,
        setter_name: String,
        initial_value: String,
    },
    Expression(String), 
}

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub items: Vec<String>,
    pub module: String,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Expression(String),
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program { imports: Vec<ImportNode>, components: Vec<ComponentNode> },
}

#[derive(Debug, Clone)]
pub struct ComponentNode {
    pub name: String,
    pub props: Vec<Prop>,
    pub body: Vec<Statement>,
    pub return_tree: UiNode,
}

// NEW: Added If and For to the enum so the Parser and Generators can use them
#[derive(Debug, Clone)]
pub enum UiNode {
    Element {
        tag: String,
        attributes: Vec<(String, AttributeValue)>,
        children: Vec<UiNode>,
    },
    Expression(String),
    If {
        condition: String,
        children: Vec<UiNode>,
    },
    For {
        item_name: String,
        collection: String,
        children: Vec<UiNode>,
    }
}

pub trait XeonGenerator {
    fn generate(&self, ast: &AstNode) -> Result<String, String>;
}