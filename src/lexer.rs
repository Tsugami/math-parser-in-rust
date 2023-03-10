use crate::token::Token;

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

    fn next_char(&mut self) -> Option<char> {
        match self.peek() {
            Some(char) => {
                self.cursor += 1;
                return Some(char);
            }
            None => None,
        }
    }

    fn peek(&mut self) -> Option<char> {
        match self.characters.get(self.cursor) {
            Some(char) => {
                return Some(*char);
            }
            None => None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let char = match self.next_char() {
            Some(char) => char,
            None => return None,
        };

        let token = match char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            char if char.is_digit(10) => {
                let mut acc = String::new();

                acc.push(char);

                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        acc.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                let num = acc.parse::<usize>().unwrap();

                Token::Number(num)
            }
            ' ' => return self.next(),
            // MOVE TO A VECTOR OF ERRORS
            // _ => return Err(format!("Unrecognized character {}", char)),
            _ => return self.next(),
        };

        println!("{:?} {:?}", token, self);

        return Some(token);
    }

    fn extract_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next() {
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

        assert_eq!(
            tokens,
            vec![Token::Number(1), Token::Plus, Token::Number(2)]
        );
    }

    #[test]
    fn full_sample() {
        let tokens = Lexer::new("1 + 2 / 3 * 4 - 1").extract_tokens().unwrap();

        use crate::lexer::Token::{Minus, Number, Plus, Slash, Star};

        assert_eq!(
            tokens,
            vec![
                Number(1),
                Plus,
                Number(2),
                Slash,
                Number(3),
                Star,
                Number(4),
                Minus,
                Number(1)
            ]
        );
    }

    #[test]
    fn big_number() {
        let tokens = Lexer::new("1230 + 24").extract_tokens().unwrap();

        use crate::lexer::Token::{Number, Plus};

        assert_eq!(tokens, vec![Number(1230), Plus, Number(24)]);
    }

    #[test]
    fn no_whitespace() {
        let tokens = Lexer::new("1230+24+1").extract_tokens().unwrap();

        use crate::lexer::Token::{Number, Plus};

        assert_eq!(
            tokens,
            vec![Number(1230), Plus, Number(24), Plus, Number(1)]
        );
    }
}
