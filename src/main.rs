use std::io::{self, stdout, Write};

struct Board {
    word: String,
    rows: [[char; 5]; 6],
}

impl Board {
    pub fn new() -> Self {
        Board {
            word: "rusty".to_string(),
            rows: [['-'; 5]; 6],
        }
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
            .position(|&r| r[0] == '-')
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
        let chars = guess.chars();
        for (idx, c) in chars.enumerate() {
            println!("{} {} {}", idx, c, c as u32);
            self.rows[index][idx] = c;
        }
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
        println!("{:?}", board.rows);
    }
}
