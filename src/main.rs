#![feature(bufreader_peek)]

use std::fs::File;

mod picojson;

fn main() {
    // let stdin = stdin();
    // let mut tokenizer = Tokenizer::new(stdin);
    let file = File::open("test.json").unwrap();
    let mut tokenizer = picojson::Tokenizer::new(file);
    let mut pretty_printer = picojson::Prettier{indents: 0, is_nl: false, is_in_arr: false};
    while let Some(tok) = tokenizer.get_token() {
        pretty_printer.print_token(tok)
    }
}