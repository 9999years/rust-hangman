use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::iter::{Copied, FromIterator, Peekable};
use std::slice;

const STATES: [&str; 7] = [
    r#"
|-----|
|
|
|
|_______
    "#,
    r#"
|-----|
|     o
|
|
|_______
    "#,
    r#"
|-----|
|     o
|     |
|
|_______
    "#,
    r#"
|-----|
|     o
|     |
|    /
|_______
    "#,
    r#"
|-----|
|     o
|     |
|    / \
|_______
    "#,
    r#"
|-----|
|     o
|    /|
|    / \
|_______
    "#,
    r#"
|-----|
|     o
|    /|\
|    / \
|_______
    "#,
];

struct Hangman<'a, 'b, I>
where
    I: Iterator<Item = &'a str>,
{
    pictures: Peekable<I>,
    word: &'b str,
    chars_in_word: HashSet<char>,
    guessed: HashSet<char>,
    partially_revealed: String,
}

impl<'b> Hangman<'static, 'b, Copied<slice::Iter<'static, &'static str>>> {
    pub fn new(word: &'b str) -> Self {
        Hangman {
            pictures: STATES.iter().copied().peekable(),
            guessed: HashSet::with_capacity(STATES.len()),
            chars_in_word: HashSet::from_iter(word.chars()),
            partially_revealed: "_".repeat(word.len()),
            word,
        }
    }
}

enum GuessResponse {
    AlreadyGuessed,
    Incorrect,
    Correct,
    Win,
    Lose,
}

impl<'a, 'b, I> Hangman<'a, 'b, I>
where
    I: Iterator<Item = &'a str>,
{
    pub fn picture(&mut self) -> Option<&'a str> {
        self.pictures.peek().copied()
    }

    pub fn partially_revealed(&self) -> &str {
        &self.partially_revealed
    }

    pub fn word(&self) -> &'b str {
        self.word
    }

    pub fn guess(&mut self, c: char) -> GuessResponse {
        if self.pictures.peek().is_none() {
            GuessResponse::Lose
        } else if self.guessed.contains(&c) {
            GuessResponse::AlreadyGuessed
        } else {
            // Not already guessed
            self.guessed.insert(c);
            if self.chars_in_word.contains(&c) {
                self.update_revealed(c);
                if self.partially_revealed == self.word {
                    GuessResponse::Win
                } else {
                    GuessResponse::Correct
                }
            } else {
                self.pictures.next();
                if self.pictures.peek().is_none() {
                    GuessResponse::Lose
                } else {
                    GuessResponse::Incorrect
                }
            }
        }
    }

    fn update_revealed(&mut self, c: char) {
        let chr_string: String = c.to_string();
        for (i, _) in self.word.char_indices().filter(|(_, chr)| c == *chr) {
            self.partially_revealed
                .replace_range(i..i + chr_string.len(), &chr_string);
        }
    }
}

fn read_alphabetic_char() -> char {
    let mut ret: char = ' ';
    while !ret.is_alphabetic() {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        if let Some(c) = line.chars().next() {
            ret = c;
        }
    }
    ret
}

fn main() {
    let words = vec![
        "snakes", "thanks", "granted", "awkward", "bagpipes", "banjo", "bungler", "croquet",
        "crypt",
    ];
    let current_word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut hangman = Hangman::new(current_word);
    loop {
        println!("{}", hangman.picture().unwrap());
        println!("{}", hangman.partially_revealed());
        println!("Make a guess!");
        match hangman.guess(read_alphabetic_char()) {
            GuessResponse::AlreadyGuessed => println!("You already guessed that!"),
            GuessResponse::Incorrect => println!("Incorrect"),
            GuessResponse::Correct => println!("Correct!\n{}", hangman.partially_revealed()),
            GuessResponse::Win => {
                println!("You win! The word was '{}'.", hangman.word());
                return;
            }
            GuessResponse::Lose => {
                println!("You lose! The word was '{}'.", hangman.word());
                return;
            }
        }
    }
}
