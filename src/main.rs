use std::string::{String, ToString};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
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
struct Token {
    value: String,
    token_type: TokenType
}

struct Tokenizer<T: Read + Seek> {
    prev_pos: usize,
    file: T,
}

impl Tokenizer<File> {
    fn new(file: File) -> Self{
        return Self {
            prev_pos: 0, file: file
        }
    }

    fn read_n<const N: usize>(&mut self) -> io::Result<String> {
        let mut buf  = [0u8; N];
        if self.file.read_exact(&mut buf).is_ok(){
        } else {
            Err::<String, ()>(()).unwrap();
        }
        let ret = std::str::from_utf8(&buf).unwrap();
        return Ok(ret.to_string())

    }

    fn read<'a>(&mut self) -> io::Result<char> {
        let mut buf  = [0u8; 1];
        if self.file.read_exact(&mut buf).is_ok(){
        } else {
            Err::<&'a char, ()>(()).unwrap();
        }
        let ret = std::str::from_utf8(&buf).unwrap();
        return Ok(ret.parse().unwrap())

    }

    fn peek(&mut self) -> io::Result<char> {
        let ret = self.read();
        self.seek(-1);
        return ret

    }

    fn pos(&mut self) -> u64 {
        return self.file.stream_position().unwrap()
    }

    fn read_until(&mut self, chars: &str) -> String {
        let mut c = '\0';
        let mut ret = String::new();
        while c == '\0' || !chars.contains(c) {
            match self.read() {
                Ok(p) => {
                    if chars.contains(p) {
                        break
                    }
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

    fn seek(&mut self, n: i64) {
        self.file.seek_relative(n).unwrap();
    }


    fn read_while(&mut self, chars: &str) -> String {
        let mut c = '\0';
        let mut ret = String::new();
        while c== '\0' || chars.contains(c) {
            match self.read() {
                Ok(p) => {
                    if !chars.contains(p) {
                        break
                    }
                    c = p;
                    ret.push(p);
                },
                Err(_) => {
                    break
                }
            }
        }
        self.seek(-1);
        return ret.to_string()
    }

    fn skip_whitespace(mut self) {
        self.read_until(" \n");
    }

    fn get_token(&mut self) -> Option<Token> {
        self.read_while(" \n");
        let p = self.pos();
        self.prev_pos = p as usize;
        match self.read() {
            Ok(p) => {
                match p {
                    'f' | 't' => {
                        let val = self.read_n::<3>().unwrap();
                        return Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Boolean,
                        })
                    },
                    '-' | '.' | '0'|  '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let val = self.read_while("-.0123456789");
                        return Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Number,
                        })
                    },
                    '\"' => {
                        let val = self.read_until("\"");
                        return Some(Token{
                            value: val,
                            token_type: TokenType::StringValue,
                        })
                    },
                    '{' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::CurlyOpen,
                        })
                    },
                    '}' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::CurlyClose,
                        })
                    },
                    '[' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::ArrayOpen,
                        })
                    },
                    ']' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::ArrayClose,
                        })
                    },
                    ':' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::Colon,
                        })
                    },
                    ',' => {
                        return Some(Token{
                            value: p.to_string(),
                            token_type: TokenType::Comma,
                        })
                    },
                    'n' => {
                        let val = self.read_n::<3>().unwrap();
                        return Some(Token{
                            value: p.to_string() + &val,
                            token_type: TokenType::Null,
                        })
                    }
                    _ => {
                        return None
                    }
                }
            }
            Err(_) => {
                return None
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_until() {
        let file = File::open("test.json").unwrap();
        let mut tokenizer = Tokenizer::new(file);
        let r = tokenizer.read_until("\"");
        assert!(r == "{".to_string())
    }

    #[test]
    fn test_peek() {
        let file = File::open("test.json").unwrap();
        let mut tokenizer = Tokenizer::new(file);
        let _ = tokenizer.read_while("{}");
        assert!(tokenizer.peek().unwrap() == '\"')
    }

    #[test]
    fn test_get_token() {
        let file = File::open("test.json").unwrap();
        let mut tokenizer = Tokenizer::new(file);
        let r = tokenizer.get_token().unwrap();
        assert_eq!(r.token_type, TokenType::CurlyOpen);
        assert_eq!(r.value, "{");
        let rr = tokenizer.get_token().unwrap();
        assert_eq!(rr.token_type, TokenType::StringValue);
        assert_eq!(rr.value, "key".to_string());
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
}

fn main() {

}