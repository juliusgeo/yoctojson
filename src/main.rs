#![feature(bufreader_peek)]
#![feature(buf_read_has_data_left)]

use std::io::stdin;
use std::fs::File;

mod yoctojson;

fn main() {
    // let stdin = stdin().lock();
    // let mut tokenizer = yoctojson::Tokenizer::new(stdin);
    let file = File::open("test_files/test.json").unwrap();
    let mut tokenizer = yoctojson::Tokenizer::new(file);
    let mut pretty_printer = yoctojson::Prettier{indents: 0, is_nl: false, is_in_arr: false};
    while let Some(tok) = tokenizer.get_token() {
        pretty_printer.print_token(tok)
    }
}