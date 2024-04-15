use std::io::{self, Write};

use crate::{parser::Parser, tokenizer::Tokenizer};

mod node;
mod parser;
mod token;
mod tokenizer;

fn main() {
    let mut line;
    let mut tokenizer;
    let mut parser;
    loop {
        print!("xcalcrs >");
        io::stdout().flush().unwrap();
        line = io::stdin().lines().next().unwrap().unwrap();
        tokenizer = Tokenizer::new(&line);
        let tokenizer_res = tokenizer.tokenize();
        if let Err(err) = tokenizer_res {
            println!("{err}");
            continue;
        }
        let tokens = tokenizer_res.unwrap();
        parser = Parser::new(&tokens);
        let parser_res = parser.parse();
        if let Err(err) = parser_res {
            println!("{err}");
            continue;
        }
        println!("{:?}", parser_res.unwrap());
    }
}
