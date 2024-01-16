use std::{env, fs};
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = fs::File::open(filename)
        .expect(&*format!("open file {} fails", filename));

    let mut chars: usize = 0;
    let mut words: usize = 0;
    let mut lines: usize = 0;

    let mut buf = String::new();
    let mut buf_reader = BufReader::new(file);
    while let Ok(code) = buf_reader.read_line(&mut buf) {
        if code == 0 { break; }
        chars += buf.len();
        words += buf.split_whitespace().count();
        lines += 1;
        buf.clear();
    }

    println!("file {}", filename);
    println!("characters: {}", chars);
    println!("words: {}", words);
    println!("lines: {}", lines);
}
