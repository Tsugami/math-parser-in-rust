use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]

enum Operator {
    Add,
    Sub,
    Multi,
    Div,
}

impl TryFrom<Token> for Operator {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Add => Ok(Operator::Add),
            Token::Sub => Ok(Operator::Sub),
            Token::Multi => Ok(Operator::Multi),
            Token::Div => Ok(Operator::Div),
            _ => Err(format!("invalid operator from token")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Number(i64),
    BinaryExpr {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub struct MathParser {}

impl MathParser {
    pub fn parse(input: &str) -> Result<i64, String> {
        let tokens = Lexer::from_str(input)?;
        let ast = Self::build_ast(tokens, 0)?;

        Ok(Self::eval(&ast))
    }

    fn infix_binding_power(token: &Token) -> i64 {
        match token {
            Token::Add => 1,
            Token::Sub => 2,
            Token::Multi => 3,
            Token::Div => 4,
            _ => 0,
        }
    }

    fn build_ast(tokens: Vec<Token>, min_precedence: i64) -> Result<Node, String> {
        let mut cursor = 0;

        while let Some(token) = tokens.get(cursor) {
            if Self::infix_binding_power(token) > min_precedence {
                let result = match token {
                    Token::Number(num) => Node::Number(*num),
                    op => {
                        let (lhs, rhs) = tokens.split_at(cursor);

                        let mut rhs: Vec<Token> = rhs.into();

                        // skip op
                        // input= 1 + 2
                        // lhs = [1] rhs = [+, 2]
                        rhs.remove(0);
                        // lhs = [1] rhs = [2]

                        let rhs = Self::build_ast(rhs.into(), min_precedence + 1)?;
                        let lhs = Self::build_ast(lhs.into(), min_precedence + 1)?;

                        let op = *op;

                        Node::BinaryExpr {
                            op: op.try_into()?,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        }
                    }
                };

                return Ok(result);
            }

            cursor += 1;
        }

        Self::build_ast(tokens, -1)
    }

    fn eval(node: &Node) -> i64 {
        match node {
            Node::Number(num) => *num,
            Node::BinaryExpr { op, left, right } => {
                let left = Self::eval(left);
                let right = Self::eval(right);
                match op {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Div => left / right,
                    Operator::Multi => left * right,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        parser::{MathParser, Node, Operator},
    };

    #[test]
    fn sample_ast() {
        let tokens = Lexer::from_str("1 + 2").unwrap();
        let ast = MathParser::build_ast(tokens, 0);

        assert_eq!(
            ast.unwrap(),
            Node::BinaryExpr {
                op: Operator::Add,
                left: Box::new(Node::Number(1)),
                right: Box::new(Node::Number(2)),
            }
        );
    }

    #[test]
    fn sample_sum() {
        let result = MathParser::parse("1 + 2").unwrap();

        assert_eq!(result, 3)
    }
}
