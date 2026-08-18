use xeon_lexer::{Lexer, Token, TokenSpan};
use xeon_ast::{AstNode, ComponentNode, ImportNode, UiNode, Statement, AttributeValue};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: TokenSpan,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError {
            message: msg.to_string(),
            line: self.current_token.line,
            col: self.current_token.col,
        }
    }

    fn token_to_str(&self, token: &Token) -> String {
        match token {
            Token::Identifier(id) => id.clone(),
            Token::StringLiteral(s) => format!("\"{}\"", s),
            Token::OpenParen => "(".to_string(),
            Token::CloseParen => ")".to_string(),
            Token::Arrow => " => ".to_string(),
            Token::Comma => ", ".to_string(),
            _ => "".to_string(),
        }
    }

    fn parse_ui_node(&mut self) -> Result<UiNode, ParseError> {
        self.advance(); // Skip `<`
        
        let tag_name = if let Token::Identifier(name) = &self.current_token.token { 
            name.clone() 
        } else { 
            return Err(self.error("Expected Tag Name after '<'")); 
        };
        self.advance();

        let mut attributes = Vec::new();

        while self.current_token.token != Token::AngleClose && self.current_token.token != Token::Slash && self.current_token.token != Token::Eof {
            if let Token::Identifier(attr_name) = self.current_token.token.clone() {
                self.advance();
                if self.current_token.token == Token::Assign {
                    self.advance();
                    if let Token::StringLiteral(attr_val) = self.current_token.token.clone() {
                        attributes.push((attr_name, AttributeValue::String(attr_val)));
                        self.advance();
                    } else if self.current_token.token == Token::OpenBrace {
                        self.advance();
                        let mut expr_str = String::new();
                        let mut brace_depth = 1;
                        while brace_depth > 0 && self.current_token.token != Token::Eof {
                            if self.current_token.token == Token::OpenBrace { brace_depth += 1; }
                            if self.current_token.token == Token::CloseBrace { brace_depth -= 1; }
                            
                            if brace_depth > 0 {
                                expr_str.push_str(&self.token_to_str(&self.current_token.token));
                                self.advance();
                            }
                        }
                        attributes.push((attr_name, AttributeValue::Expression(expr_str)));
                        self.advance(); // skip }
                    } else {
                        return Err(self.error("Expected string literal or expression after '='"));
                    }
                }
            } else { 
                return Err(self.error("Expected attribute name")); 
            }
        }

        if self.current_token.token == Token::Slash {
            self.advance(); self.advance(); // Skip `/>`
            return Ok(UiNode::Element { tag: tag_name, attributes, children: vec![] });
        }

        self.advance(); // Skip `>`

        let mut children = Vec::new();
        while self.current_token.token != Token::Eof {
            if self.current_token.token == Token::AngleOpen {
                if self.lexer.peek_token().token == Token::Slash { 
                    break; 
                } else { 
                    children.push(self.parse_ui_node()?); 
                }
            } else if self.current_token.token == Token::OpenBrace {
                self.advance();
                if let Token::Identifier(expr) = &self.current_token.token {
                    children.push(UiNode::Expression(expr.clone()));
                }
                self.advance(); self.advance(); // skip ident and }
            } else { 
                self.advance(); 
            }
        }

        // Validate Closing Tag matches Opening Tag
        self.advance(); // `<`
        self.advance(); // `/`
        if let Token::Identifier(closing_tag) = &self.current_token.token {
            if closing_tag != &tag_name {
                return Err(self.error(&format!("Mismatched closing tag. Expected </{}>, found </{}>", tag_name, closing_tag)));
            }
        } else {
            return Err(self.error("Expected closing tag name"));
        }
        
        self.advance(); // skip name
        self.advance(); // skip `>`

        // Intercept Control Flow: <If>
        if tag_name == "If" {
            let mut condition = "false".to_string();
            for (k, v) in &attributes {
                if k == "condition" {
                    condition = match v {
                        AttributeValue::Expression(e) => e.clone(),
                        AttributeValue::String(s) => format!("\"{}\"", s),
                    };
                }
            }
            return Ok(UiNode::If { condition, children });
        }

        // Intercept Control Flow: <For>
        if tag_name == "For" {
            let mut item_name = "item".to_string();
            let mut collection = "[]".to_string();
            for (k, v) in &attributes {
                if k == "as" {
                    item_name = match v {
                        AttributeValue::String(s) => s.clone(),
                        AttributeValue::Expression(e) => e.clone(),
                    };
                }
                if k == "each" {
                    collection = match v {
                        AttributeValue::Expression(e) => e.clone(),
                        AttributeValue::String(s) => s.clone(), 
                    };
                }
            }
            return Ok(UiNode::For { item_name, collection, children });
        }

        Ok(UiNode::Element { tag: tag_name, attributes, children })
    }

    pub fn parse_program(&mut self) -> Result<AstNode, ParseError> {
        let mut components = Vec::new();
        let mut imports = Vec::new();
        
        while self.current_token.token != Token::Eof {
            match self.current_token.token {
                Token::Import => {
                    self.advance(); self.advance(); 
                    let mut items = Vec::new();
                    while self.current_token.token != Token::CloseBrace {
                        if let Token::Identifier(item) = &self.current_token.token { items.push(item.clone()); }
                        self.advance();
                    }
                    self.advance(); self.advance(); 
                    let module = if let Token::StringLiteral(mod_name) = &self.current_token.token { mod_name.clone() } else { "".to_string() };
                    self.advance(); self.advance();
                    imports.push(ImportNode { items, module });
                }
                Token::Component => {
                    self.advance();
                    if let Token::Identifier(comp_name) = self.current_token.token.clone() {
                        self.advance();
                        while self.current_token.token != Token::OpenBrace && self.current_token.token != Token::Eof { self.advance(); }
                        self.advance(); 

                        let mut body = Vec::new();
                        let mut return_tree = UiNode::Element { tag: "Error".to_string(), attributes: vec![], children: vec![] };

                        while self.current_token.token != Token::CloseBrace && self.current_token.token != Token::Eof {
                            match &self.current_token.token {
                                Token::Let => {
                                    self.advance(); self.advance(); 
                                    let state_name = if let Token::Identifier(id) = &self.current_token.token { id.clone() } else { "".to_string() };
                                    self.advance(); self.advance(); 
                                    let setter_name = if let Token::Identifier(id) = &self.current_token.token { id.clone() } else { "".to_string() };
                                    
                                    for _ in 0..5 { self.advance(); }
                                    let initial_value = if let Token::StringLiteral(s) = &self.current_token.token { format!("\"{}\"", s) } else { "null".to_string() };
                                    
                                    while self.current_token.token != Token::Semicolon && self.current_token.token != Token::Eof { self.advance(); }
                                    self.advance(); 

                                    body.push(Statement::StateDeclaration { state_name, setter_name, initial_value });
                                }
                                Token::Return => {
                                    self.advance(); 
                                    return_tree = self.parse_ui_node()?;
                                    if self.current_token.token == Token::Semicolon { self.advance(); }
                                }
                                _ => self.advance(),
                            }
                        }
                        components.push(ComponentNode { name: comp_name, props: vec![], body, return_tree });
                    }
                }
                _ => self.advance(),
            }
        }
        Ok(AstNode::Program { imports, components })
    }
}