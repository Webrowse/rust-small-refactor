use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: word_count <file>");
        return;
    }

    let path = &args[1];
    let contents = fs::read_to_string(path).unwrap();

    let count = count_words(&contents);

    println!("word count: {}", count);
}

fn count_words(input: &str) -> usize {
    let mut count = 0;

    for word in input.split_whitespace() {
        if word.len() > 0 {
            count += 1;
        }
    }

    count
}
