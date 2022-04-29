use std::fmt;
use std::io::{self, stdout, Write};

struct Board {
    word: String,
    rows: [[Cell; 5]; 6],
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Green(char),
    Yellow(char),
    Gray(char),
    Empty,
}

impl Cell {
    pub fn new() -> Self {
        Cell::Empty
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            word: "rusty".to_string(),
            rows: [[Cell::new(); 5]; 6],
        }
    }
    pub fn guess_cell(&self, guess_idx: usize, c: char) -> Cell {
        let mut existing = false;
        for (word_idx, letter) in self.word.chars().enumerate() {
            if guess_idx == word_idx && c == letter {
                return Cell::Green(c);
            } else if guess_idx != word_idx && c == letter {
                return Cell::Yellow(c);
            }
            if c == letter {
                existing = true;
            }
        }
        if !existing {
            return Cell::Gray(c);
        }
        return Cell::Empty;
    }
    // self, &self, mut self, &mut self
    pub fn guess(&mut self, guess: &str) {
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        let index = self
            .rows
            .iter()
            .position(|&r| r[0] == Cell::Empty)
            .expect("You lose!");
        // ['r', 'o', 'u', 't', 'e']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        // ['-', '-', '-', '-', '-']
        let guess = guess.trim();
        if guess.len() > 5 {
            panic!("can't guess more than 5")
        }
        // atlye
        let chars = guess.chars();
        for (guess_idx, c) in chars.enumerate() {
            println!("{} {} {}", guess_idx, c, c as u32);
            let cell = self.guess_cell(guess_idx, c);
            self.rows[index][guess_idx] = cell;
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.rows {
            for c in r {
                write!(f, "{:?} | ", c);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    loop {
        let mut input = String::new();
        print!("Make a guess: ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read guess");

        board.guess(&input);
        println!("{:?}", board);
    }
}
