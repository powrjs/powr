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

        let tokenizer = Tokenizer::new(chars);

        for token in tokenizer {
            println!("{:#?}", token);
        }

        println!();
    });
}
