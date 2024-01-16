// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::iter::FromIterator;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    // let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    // let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let ans = pick_a_random_word();
    let mut disp = vec!['-'; ans.len()];
    let mut tries = NUM_INCORRECT_GUESSES;

    let mut guess = String::new();
    let mut guessed = String::new();
    loop {
        print!("Please guess a letter: ");
        io::stdout().flush()
            .expect("Error flushing stdout");
        guess.clear();
        io::stdin().read_line(&mut guess)
            .expect("Error reading line.");

        if guess.len() > 3 {
            // including /r and /n
            println!("Invalid input!");
            continue;
        }

        let ch = char::from(guess.as_bytes()[0]);
        guessed.push(ch);
        let matches: Vec<_> = ans.match_indices(ch).collect();
        if matches.is_empty() {
            println!("Sorry, that letter is not in the word");
            tries -= 1;
        }
        else {
            matches.iter().for_each( |&(idx, _)| disp[idx] = ch );
            print!("The word so far is ");
            disp.iter().for_each( |&x| print!("{}", x) );
            println!();
        }
        if tries <= 0 {
            println!("Sorry, you ran out of guesses!");
            println!("The answer is: {}", ans);
            break;
        }
        if ans == String::from_iter(&disp) {
            println!("Congratulations you guessed the secret word: {}!", ans);
            break;
        }
        println!("You have guessed the following letters: {}", guessed);
        println!("You have {} guesses left", tries);
        println!();
    }

}
