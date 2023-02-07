#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    Multi,
    Div,
    Number(i64),
}

#[derive(Debug, PartialEq)]
pub struct Lexer {
    cursor: usize,
    characters: Vec<char>,
}

impl Lexer {
    pub fn from_str(input: &str) -> Result<Vec<Token>, String> {
        Self::new(input).extract_tokens()
    }

    fn new(input: &str) -> Self {
        Self {
            cursor: 0,
            characters: input.chars().collect(),
        }
    }

    fn extract_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let char = match self.characters.get(self.cursor) {
                Some(char) => char,
                None => break,
            };

            self.cursor += 1;

            let token = match char {
                ' ' => continue,
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Multi,
                '/' => Token::Div,
                char if char.is_digit(10) => {
                    self.cursor -= 1;

                    let mut acc = String::new();

                    while let Some(c) = self.characters.get(self.cursor) {
                        if c.is_digit(10) {
                            acc.push(*c);
                            self.cursor += 1;
                        } else {
                            break;
                        }
                    }

                    let num = acc.parse::<i64>().unwrap();

                    Token::Number(num)
                }
                _ => return Err(format!("Unrecognized character {}", char)),
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn new_lexer() {
        assert_eq!(
            Lexer::new("1 + 2"),
            Lexer {
                cursor: 0,
                characters: vec!['1', ' ', '+', ' ', '2']
            }
        );
    }

    #[test]
    fn sample_sum() {
        let tokens = Lexer::new("1 + 2").extract_tokens().unwrap();

        assert_eq!(tokens, vec![Token::Number(1), Token::Add, Token::Number(2)]);
    }

    #[test]
    fn full_sample() {
        let tokens = Lexer::new("1 + 2 / 3 * 4 - 1").extract_tokens().unwrap();

        use crate::lexer::Token::{Add, Div, Multi, Number, Sub};

        assert_eq!(
            tokens,
            vec![
                Number(1),
                Add,
                Number(2),
                Div,
                Number(3),
                Multi,
                Number(4),
                Sub,
                Number(1)
            ]
        );
    }

    #[test]
    fn big_number() {
        let tokens = Lexer::new("1230 + 24").extract_tokens().unwrap();

        use crate::lexer::Token::{Add, Number};

        assert_eq!(tokens, vec![Number(1230), Add, Number(24)]);
    }

    #[test]
    fn no_whitespace() {
        let tokens = Lexer::new("1230+24+1").extract_tokens().unwrap();

        use crate::lexer::Token::{Add, Number};

        assert_eq!(tokens, vec![Number(1230), Add, Number(24), Add, Number(1)]);
    }
}
