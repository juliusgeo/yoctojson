
use std::string::{String, ToString};
use std::fs::File;
use std::io;
use std::io::{BufReader};
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    CurlyOpen,
    CurlyClose,
    Colon,
    StringValue,
    Number,
    ArrayOpen,
    ArrayClose,
    Comma,
    Boolean,
    Null
}
pub struct Token {
    pub value: String,
    token_type: TokenType
}
#[derive(Clone)]
struct Char {
    char: char,
    is_escaped: bool
}

pub struct Tokenizer<T: Read> {
    prev_pos: usize,
    buffer: BufReader<T>,
}

impl<T: Read> Tokenizer<T> {
    pub fn new(buf: T) -> Self{
        return Self {
            prev_pos: 0, buffer: BufReader::new(buf)
        }
    }

    fn read_n<const N: usize>(&mut self) -> io::Result<String> {
        let mut buf  = [0u8; N];
        self.buffer.read_exact(&mut buf);
        let ret = std::str::from_utf8(&buf);
        match ret {
            Ok(val) => { return Ok(val.to_string()) },    // If result is Ok, return the value
            Err(err) => {
                return Ok("".to_string());
            }
        };
    }

    fn read<'a>(&mut self) -> io::Result<Char> {
        let mut buf  = [0u8; 1];
        self.buffer.read_exact(&mut buf);
        let ret = buf[0] as char;
        if ret == '\\' {
            // there's a slash, so maybe the next char is being escaped
            self.buffer.read_exact(&mut buf);
            let escaped_char = buf[0] as char;
            return match escaped_char {
                'n' => Ok(Char{char: '\n', is_escaped: true}),
                'r' => Ok(Char{char: '\r', is_escaped: true}),
                't' => Ok(Char{char: '\t', is_escaped: true}),
                '0' => Ok(Char{char: '\0', is_escaped: true}),
                '\"' => Ok(Char{char: '\"', is_escaped: true}),
                '\'' => Ok(Char{char: '\'', is_escaped: true}),
                '\\' => Ok(Char{char: '\\', is_escaped: true}),
                // if it's not actually an escaped char, we still want to add a / before it
                // so set is_escaped to true
                _ => Ok(Char{char: escaped_char, is_escaped: true})
            };
        }
        return Ok(Char{char: ret, is_escaped: false})

    }

    fn read_until(&mut self, chars: &str) -> String {
        let mut c: Char = Char{char: '\0', is_escaped: false};
        let mut ret: Vec<u8> = Vec::new();
        while c.char == '\0' || !chars.contains(c.char) || c.is_escaped {
            match self.read() {
                Ok(p) => {
                    c = p.clone();
                    if p.is_escaped {
                        ret.push('\\' as u8);
                    }
                    ret.push(p.char as u8);
                },
                Err(_) => {
                    break
                }
            }
        }
        return String::from_utf8(ret).unwrap()
    }


    fn read_while(&mut self, chars: &str) -> String {
        let mut c = '\0';
        let mut ret = String::new();
        while c== '\0' || chars.contains(c) {
            match self.buffer.peek(1) {
                Ok(p) => {
                    if p.len() != 1 {
                        break
                    }
                    let p = p[0] as char;
                    if !chars.contains(p){
                        break
                    }
                    self.buffer.consume(1);
                    c = p;
                    ret.push(p);
                },
                Err(_) => {
                    break
                }
            }
        }
        return ret.to_string()
    }

    fn skip_whitespace(mut self) {
        self.read_until(" \n");
    }

    pub fn get_token(&mut self) -> Option<Token> {
        self.read_while(" \n");
        match self.read() {
            Ok(c) => {
                let p = c.char;
                match p {
                    't' => {
                        let val = self.read_n::<3>().unwrap();
                        Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Boolean,
                        })
                    },
                    'f' => {
                        let val = self.read_n::<4>().unwrap();
                        Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Boolean,
                        })
                    },
                    '-' | '.' | '0'|  '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let val = self.read_while("-.0123456789");
                        Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Number,
                        })
                    },
                    '\"' | '\'' => {
                        let val = self.read_until(p.to_string().as_str());
                        Some(Token{
                            value: p.to_string()+&val,
                            token_type: TokenType::StringValue,
                        })
                    },
                    '{' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::CurlyOpen,
                        })
                    },
                    '}' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::CurlyClose,
                        })
                    },
                    '[' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::ArrayOpen,
                        })
                    },
                    ']' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::ArrayClose,
                        })
                    },
                    ':' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::Colon,
                        })
                    },
                    ',' => {
                        Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::Comma,
                        })
                    },
                    'n' => {
                        let val = self.read_n::<3>().unwrap();
                        Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Null,
                        })
                    }
                    _ => {
                        None
                    }
                }
            }
            Err(_) => {
                None
            }
        }

    }
}

pub struct Prettier {
    pub indents: usize,
    pub is_nl: bool,
    pub is_in_arr: bool,
}

impl Prettier {
    pub fn print_token(&mut self, token: Token) {
        match token.token_type {
            TokenType::CurlyClose => {
                self.indents -= 1;
                print!("\n{:}{:}", "\t".repeat(self.indents), token.value);
            },
            TokenType::CurlyOpen => {
                print!(" {:}\n", token.value);
                self.indents += 1;
                self.is_nl = true
            },
            TokenType::Comma => {
                if !self.is_in_arr {
                    print!("{:}\n", token.value);
                    self.is_nl = true
                } else {
                    print!("{:} ", token.value);
                }
            }
            TokenType::ArrayOpen => {
                print!("{:}", token.value);
                self.is_in_arr = true
            },
            TokenType::ArrayClose => {
                print!("{:}", token.value);
                self.is_in_arr = false
            },
            _ => {
                if self.is_nl {
                    print!("{:}{:}", "\t".repeat(self.indents), token.value);
                    self.is_nl = false
                } else {
                    print!("{:}", token.value);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};
    use crate::yoctojson::TokenType::{ArrayOpen, Colon, CurlyOpen, StringValue};

    #[test]
    fn test_read_until() {
        let file = File::open("test_files/test.json").unwrap();
        let mut tokenizer = Tokenizer::new(file);
        let r = tokenizer.read_until("\"");
        assert!(r == "{\"")
    }

    #[test]
    fn test_get_token() {
        let file = File::open("test_files/test.json").unwrap();
        let mut tokenizer = Tokenizer::new(file);
        let r = tokenizer.get_token().unwrap();
        assert_eq!(r.token_type, TokenType::CurlyOpen);
        assert_eq!(r.value, "{");
        let rr = tokenizer.get_token().unwrap();
        assert_eq!(rr.token_type, TokenType::StringValue);
        assert_eq!(rr.value, "\"key\"".to_string());
        let colon = tokenizer.get_token().unwrap();
        assert_eq!(colon.token_type, TokenType::Colon);
        assert_eq!(colon.value, ":".to_string());
        let num = tokenizer.get_token().unwrap();
        assert_eq!(num.token_type, TokenType::Number);
        assert_eq!(num.value, "1.001".to_string());
        tokenizer.get_token().unwrap();
        tokenizer.get_token().unwrap();
        tokenizer.get_token().unwrap();
        let bool = tokenizer.get_token().unwrap();
        assert_eq!(bool.token_type, TokenType::Boolean);
        assert_eq!(bool.value, "true".to_string());
        let comma = tokenizer.get_token().unwrap();
        assert_eq!(comma.token_type, TokenType::Comma);
        assert_eq!(comma.value, ",".to_string())
    }

    fn reader_from_str(s: &str) -> BufReader<Cursor<&[u8]>> {
        BufReader::new(Cursor::new(s.as_bytes()))
    }

    #[test]
    fn test_escape() {
        let mut reader = reader_from_str("[\"y\no\0c\\\t\"]");
        let mut tokenizer = Tokenizer::new(&mut reader);
        assert_eq!(tokenizer.get_token().unwrap().token_type, ArrayOpen);
        let str_token = tokenizer.get_token().unwrap();
        assert_eq!(str_token.token_type, StringValue);
        assert_eq!(str_token.value, "\"y\no\0c\\\t\"")
    }

    #[test]
    fn test_unicode_basic() {
        let mut reader = reader_from_str("[\"Здравствуйте\"]");
        let mut tokenizer = Tokenizer::new(&mut reader);
        assert_eq!(tokenizer.get_token().unwrap().token_type, ArrayOpen);
        let str_token = tokenizer.get_token().unwrap();
        assert_eq!(str_token.token_type, StringValue);
        assert_eq!(str_token.value, "\"Здравствуйте\"")
    }

    #[test]
    fn test_unicode_keys_values() {
        let mut reader = reader_from_str("{\"Здравствуйте\": [\"Здравствуйте\"]}");
        let mut tokenizer = Tokenizer::new(&mut reader);
        assert_eq!(tokenizer.get_token().unwrap().token_type, CurlyOpen);
        let str_token = tokenizer.get_token().unwrap();
        assert_eq!(str_token.token_type, StringValue);
        assert_eq!(str_token.value, "\"Здравствуйте\"")
    }
}