use power_tokenizer::token::Token;
use std::{env, process};

use power_tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("please provide a string of characters!");
        process::exit(1);
    }

    (1..args.len()).into_iter().for_each(|i| {
        let string = args.get(i).unwrap().to_owned();
        let chars: Vec<char> = string.chars().collect();

        let mut tokenizer = Tokenizer::new(chars);
        let mut last_char = Token::Illegal;

        while last_char != Token::EndOfFile {
            last_char = tokenizer.next();
            println!("{:#?}", last_char);
        }

        println!();
    });
}
