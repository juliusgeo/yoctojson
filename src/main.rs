#![feature(bufreader_peek)]

use crate::yoctojson::Tokenizer;
use std::fs::File;
use std::io::stdin;
use std::io::Read;

mod yoctojson;

fn prettify<T: Read>(mut tokenizer: Tokenizer<T>) {
    let mut pretty_printer = yoctojson::Prettier {
        indents: 0,
        is_nl: false,
        is_in_arr: false,
    };
    while let Some(tok) = tokenizer.get_token() {
        pretty_printer.print_token(tok)
    }
}
fn main() {
    let paths = std::env::args().nth(1);
    match paths {
        Some(path) => {
            let file = File::open(path).unwrap();
            let tokenizer = Tokenizer::new(file);
            prettify(tokenizer);
        }
        None => {
            let stdin = stdin().lock();
            let tokenizer = Tokenizer::new(stdin);
            prettify(tokenizer);
        }
    }
}
