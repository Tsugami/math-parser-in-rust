use std::fmt;

use crate::{lexer::Lexer, token::Token};

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Multi,
    Div,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Multi => "*",
            Operator::Div => "/",
        };

        write!(f, "{}", op)
    }
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
        let ast = Self::build_ast(tokens, None)?;

        Ok(Self::eval(&ast))
    }

    fn token_priority(token: &Token) -> i64 {
        match token {
            Token::Add => 1,
            Token::Sub => 2,
            Token::Multi => 3,
            Token::Div => 4,
            _ => 0,
        }
    }

    fn ast_to_source_with_params(ast: &Node) -> String {
        match ast {
            Node::Number(num) => num.to_string(),
            Node::BinaryExpr { op, left, right } => {
                let left = Self::ast_to_source_with_params(left);
                let right = Self::ast_to_source_with_params(right);
                format!("({left} {op} {right})")
            }
        }
    }

    fn build_ast(tokens: Vec<Token>, priority: Option<i64>) -> Result<Node, String> {
        let mut cursor = 0;

        let initial_priority = 1;

        let priority = match priority {
            Some(v) => v,
            _ => initial_priority,
        };

        while let Some(token) = tokens.get(cursor) {
            if Self::token_priority(token) == priority {
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

                        let rhs = Self::build_ast(rhs.into(), Some(priority))?;
                        let lhs = Self::build_ast(lhs.into(), Some(priority))?;

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

        let priority = if priority > 4 { 0 } else { priority + 1 };

        Self::build_ast(tokens, Some(priority))
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
                    Operator::Multi => left * right,
                    Operator::Div => left / right,
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
        let ast = MathParser::build_ast(tokens, None);

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
    fn complexly_ast() {
        let tokens = Lexer::from_str("1 + 3 - 1 + 2").unwrap();
        let ast = MathParser::build_ast(tokens, None);

        use crate::parser::Node::{BinaryExpr, Number};
        use crate::parser::Operator::{Add, Sub};

        assert_eq!(
            ast.unwrap(),
            BinaryExpr {
                op: Add,
                left: Box::new(Number(1)),
                right: Box::new(BinaryExpr {
                    op: Add,
                    left: Box::new(BinaryExpr {
                        op: Sub,
                        left: Box::new(Number(3)),
                        right: Box::new(Number(1))
                    }),
                    right: Box::new(Number(2))
                })
            }
        );
    }

    #[test]
    fn sample_sum() {
        let result = MathParser::parse("1 + 2").unwrap();

        assert_eq!(result, 3)
    }

    // #[test]
    // fn complexity_sum() {
    //     let result = MathParser::parse("4 - 32 + 1 - 34 / 5 * 10 + 140").unwrap();

    //     assert_eq!(result, 45)
    // }
}
