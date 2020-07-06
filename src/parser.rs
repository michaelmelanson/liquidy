use std::str::Chars;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedCharacter(char),
}

#[derive(Debug, Default)]
pub struct Intermediate {
    pub nodes: Vec<Node>,
}

impl Intermediate {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub enum Node {
    Span(String),
    Expression { keypath: String },
}

pub fn parse_template(template: String) -> Result<Intermediate, ParseError> {
    let mut intermediate = Intermediate::default();

    let mut current_span = String::new();
    let mut chars = template.chars();

    'main: loop {
        match chars.next() {
            None => break 'main, // end of file

            Some('{') => match chars.next() {
                None => break,
                Some('{') => {
                    intermediate.add_node(Node::Span(current_span.clone()));
                    current_span.clear();

                    intermediate.add_node(parse_expression(&mut chars)?);
                }
                Some(c) => {
                    current_span.push('{');
                    current_span.push(c);
                }
            },
            Some(c) => current_span.push(c),
        }
    }

    if current_span.len() > 0 {
        intermediate.add_node(Node::Span(current_span.clone()));
        current_span.clear();
    }

    Ok(intermediate)
}

fn parse_expression(chars: &mut Chars) -> Result<Node, ParseError> {
    let mut expression = String::new();

    'expression: loop {
        match chars.next() {
            None => return Err(ParseError::UnexpectedEOF),
            Some('}') => match chars.next() {
                None => return Err(ParseError::UnexpectedEOF),
                Some('}') => break 'expression,
                Some(c) => return Err(ParseError::UnexpectedCharacter(c)),
            },

            Some(c) => expression.push(c),
        }
    }

    Ok(Node::Expression {
        keypath: expression.trim().to_string(),
    })
}
