use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    next_position: usize,
    char: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            next_position: 0,
            char: 0,
        };

        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input.as_bytes()[self.next_position];
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.next_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.next_position];
        }
    }

    fn next_char_is(&mut self, char: u8) -> bool {
        self.peek_char() == char
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.char {
                b' ' | b'\t' => {
                    self.read_char();
                } 
                _ => {
                    break;
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.char {
            b'=' => {
                if self.next_char_is(b'=') {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.next_char_is(b'=') {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang 
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => {
                if self.next_char_is(b'=') {
                    self.read_char();
                    Token::LtEq
                } else {
                   Token::Lt 
                }
            }
            b'>' => {
                if self.next_char_is(b'=') {
                    self.read_char();
                    Token::GtEq
                } else {
                   Token::Gt 
                }
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'"' => {
                return self.read_string();
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.read_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            b'\n' => {
                if self.next_char_is(b'\n') {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        return tok;
    }

    fn read_identifier(&mut self) -> Token {
        let start_position = self.position;

        loop {
            match self.char {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_position..self.position];

        match literal {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident { name: String::from(literal) }
        }
    }

    fn read_number(&mut self) -> Token {
        let start_position = self.position;

        loop {
            match self.char {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _  => {
                    break;
                }
            }
        }

        let literal = &self.input[start_position..self.position];

        Token::Int(literal.parse::<i64>().unwrap())
    }

    fn read_string(&mut self) -> Token {
        self.read_char();

        let start_position = self.position;

        loop {
            match self.char {
                b'"' | 0 => {
                    let literal = &self.input[start_position..self.position];
                    self.read_char();
                    return Token::String(literal.to_string());
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"let five=5;"#;

        let tests = vec![
            Token::Let,
            Token::Ident{name: String::from("five")},
            Token::Assign,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}