mod dictionary;
mod utils;

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

use dictionary::Dictionary;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    style::Color,
    terminal::{self, ClearType},
    Command, Result,
};

struct Board {
    word: String,
    word_count: HashMap<char, i32>,
    rows: [[Cell; 5]; 6],
    dictionary: dictionary::Dictionary,
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

fn count_chars(a: &str) -> HashMap<char, i32> {
    let mut count: HashMap<char, i32> = HashMap::new(); // {'a': 0}
    for letter in a.chars() {
        let letter_count = count.entry(letter).or_insert(0);
        *letter_count += 1;
    }
    count
}

impl Board {
    // self, &self, &mut self
    pub fn new(word: String) -> Self {
        let word_count = count_chars(&word);
        Board {
            word: word,
            word_count: word_count,
            rows: [[Cell::new(); 5]; 6],
            dictionary: Dictionary::new("./data/dictionary.txt"),
        }
    }

    fn mark_green(&mut self, index: usize, guess: &str) {
        let chars = guess.chars();
        for (guess_idx, c) in chars.enumerate() {
            for (word_idx, letter) in self.word.chars().enumerate() {
                let count = self.word_count.get_mut(&letter).unwrap();
                if guess_idx == word_idx && c == letter {
                    *count -= 1;
                    self.rows[index][guess_idx] = Cell::Green(letter);
                }
            }
        }
    }

    fn mark_yellow(&mut self, index: usize, guess: &str) {
        let chars = guess.chars();
        for (guess_idx, c) in chars.enumerate() {
            for (word_idx, letter) in self.word.chars().enumerate() {
                let count = self.word_count.get_mut(&letter).unwrap();
                if *count > 0 && guess_idx != word_idx && c == letter {
                    *count -= 1;
                    self.rows[index][guess_idx] = Cell::Yellow(letter);
                }
            }
        }
    }

    fn mark_gray(&mut self, index: usize, guess: &str) {
        let chars = guess.chars();
        for (guess_idx, c) in chars.enumerate() {
            let cell = self.rows[index][guess_idx];
            if cell == Cell::Empty {
                self.rows[index][guess_idx] = Cell::Gray(c);
            }
        }
    }

    // self, &self, mut self, &mut self
    pub fn guess(&mut self, guess: &str) {
        let index = self
            .rows
            .iter()
            .position(|&r| r[0] == Cell::Empty)
            .expect("You lose!");
        let guess = guess.trim();
        if guess.len() > 5 {
            panic!("can't guess more than 5")
        }
        self.word_count = count_chars(&self.word);
        self.mark_green(index, guess);
        self.mark_yellow(index, guess);
        self.mark_gray(index, guess);
    }

    pub fn print(&self) {
        terminal::enable_raw_mode(); // check for error
        queue!(
            io::stdout(),
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        );

        // INSERT CODE HERE
        for (idx, r) in self.rows.iter().enumerate() {
            queue!(
                io::stdout(),
                style::ResetColor,
                cursor::Hide,
                cursor::MoveTo(0, idx.try_into().unwrap())
            );
            for c in r {
                match c {
                    Cell::Green(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Green),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        );
                    }
                    Cell::Yellow(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Yellow),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        );
                    }
                    Cell::Gray(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Grey),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        );
                    }
                    Cell::Empty => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::DarkGrey),
                            style::Print("     ")
                        );
                    }
                }
            }
        }
        queue!(io::stdout(), style::ResetColor);

        // END CODE
        io::stdout().flush();
        terminal::disable_raw_mode();
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
    let mut board = Board::new("rusty".to_string());
    loop {
        let mut input = String::new();
        print!("\nMake a guess: ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read guess");

        if !board.dictionary.is_a_word(&input) {
            println!("{} is not in the dictionary!", &input.trim());
        } else {
            board.guess(&input);
            board.print();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;
    use crate::Cell;

    #[test]
    fn green() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rusty");
        assert_eq!(
            board.rows[0],
            [
                Cell::Green('r'),
                Cell::Green('u'),
                Cell::Green('s'),
                Cell::Green('t'),
                Cell::Green('y')
            ]
        );
    }

    #[test]
    fn fail_case() {
        let mut board = Board::new("rusty".to_string());
        board.guess("tests");
        assert_eq!(
            board.rows[0],
            [
                Cell::Gray('t'),
                Cell::Gray('e'),
                Cell::Green('s'),
                Cell::Green('t'),
                Cell::Gray('s')
            ]
        );
    }

    #[test]
    fn yellow() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rutsy");
        assert_eq!(
            board.rows[0],
            [
                Cell::Green('r'),
                Cell::Green('u'),
                Cell::Yellow('t'),
                Cell::Yellow('s'),
                Cell::Green('y')
            ]
        );
    }

    #[test]
    fn gray() {
        let mut board = Board::new("rusty".to_string());
        board.guess("abcde");
        assert_eq!(
            board.rows[0],
            [
                Cell::Gray('a'),
                Cell::Gray('b'),
                Cell::Gray('c'),
                Cell::Gray('d'),
                Cell::Gray('e')
            ]
        );
    }

    #[test]
    fn already_used_green() {
        let mut board = Board::new("rusty".to_string());
        board.guess("ruuty");
        assert_eq!(
            board.rows[0],
            [
                Cell::Green('r'),
                Cell::Green('u'),
                Cell::Gray('u'),
                Cell::Green('t'),
                Cell::Green('y')
            ]
        );
    }

    #[test]
    fn already_used_green_first() {
        let mut board = Board::new("rusty".to_string());
        board.guess("uusty");
        assert_eq!(
            board.rows[0],
            [
                Cell::Gray('u'),
                Cell::Green('u'),
                Cell::Green('s'),
                Cell::Green('t'),
                Cell::Green('y')
            ]
        );
    }

    #[test]
    fn guess_the_same_thing() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rutsy");
        board.guess("rutsy");
        assert_eq!(
            board.rows[0],
            [
                Cell::Green('r'),
                Cell::Green('u'),
                Cell::Yellow('t'),
                Cell::Yellow('s'),
                Cell::Green('y')
            ]
        );
        assert_eq!(
            board.rows[1],
            [
                Cell::Green('r'),
                Cell::Green('u'),
                Cell::Yellow('t'),
                Cell::Yellow('s'),
                Cell::Green('y')
            ]
        );
    }
}
