use std::io::{self, Write};

use crate::tokenizer::Tokenizer;

mod token;
mod tokenizer;

fn main() {
    let mut line;
    let mut tokenizer;
    loop {
        print!("xcalcrs >");
        io::stdout().flush().unwrap();
        line = io::stdin().lines().next().unwrap().unwrap();
        tokenizer = Tokenizer::new(&line);
        match tokenizer.tokenize() {
            Ok(tokens) => println!("{tokens:?}"),
            Err(err) => println!("{err}"),
        }
    }
}
