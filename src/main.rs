use std::io::{self, Write};

use crate::{interpreter::Interpreter, parser::Parser, tokenizer::Tokenizer};

mod interpreter;
mod node;
mod parser;
mod token;
mod tokenizer;

fn main() {
    let mut line;
    let mut tokenizer;
    let mut parser;
    let interpreter = Interpreter::new();
    print!("Solve systems of two equations? (y/n) >");
    io::stdout().flush().unwrap();
    line = io::stdin().lines().next().unwrap().unwrap();
    if line == "y" {
        loop {
            println!("xcalcrs");
            print!("┌");
            io::stdout().flush().unwrap();
            line = io::stdin().lines().next().unwrap().unwrap();
            tokenizer = Tokenizer::new(&line);
            let mut tokenizer_res = tokenizer.tokenize();
            if let Err(err) = tokenizer_res {
                println!("{err}");
                continue;
            }
            let mut tokens = tokenizer_res.unwrap();
            parser = Parser::new(&tokens);
            let mut parser_res = parser.parse();
            if let Err(err) = parser_res {
                println!("{err}");
                continue;
            }
            let mut node = parser_res.unwrap();
            let mut interpreter_res = interpreter.visit(&node, None);
            if let Err(err) = interpreter_res {
                println!("{err}");
                continue;
            }
            let eq1 = interpreter_res.unwrap();
            print!("└");
            io::stdout().flush().unwrap();
            line = io::stdin().lines().next().unwrap().unwrap();
            tokenizer = Tokenizer::new(&line);
            tokenizer_res = tokenizer.tokenize();
            if let Err(err) = tokenizer_res {
                println!("{err}");
                continue;
            }
            tokens = tokenizer_res.unwrap();
            parser = Parser::new(&tokens);
            parser_res = parser.parse();
            if let Err(err) = parser_res {
                println!("{err}");
                continue;
            }
            node = parser_res.unwrap();
            interpreter_res = interpreter.visit(&node, None);
            if let Err(err) = interpreter_res {
                println!("{err}");
                continue;
            }
            let eq2 = interpreter_res.unwrap();
            print!("Guess x >");
            io::stdout().flush().unwrap();
            line = io::stdin().lines().next().unwrap().unwrap();
            let mut guess = (0f64, 0f64);
            guess.0 = line.parse().unwrap();
            print!("Guess y >");
            io::stdout().flush().unwrap();
            line = io::stdin().lines().next().unwrap().unwrap();
            guess.1 = line.parse().unwrap();
            let system_res = interpreter.solve_system(&eq1, &eq2, guess);
            if let Err(err) = system_res {
                println!("{err}");
                continue;
            }
            let coords = system_res.unwrap();
            println!("(x,y) = ({:.4}, {:.4})", coords.0, coords.1);
        }
    } else {
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
            let node = parser_res.unwrap();
            let interpreter_res = interpreter.visit(&node, None);
            if let Err(err) = interpreter_res {
                println!("{err}");
                continue;
            }
            println!("{}", interpreter_res.unwrap());
        }
    }
}
