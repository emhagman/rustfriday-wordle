use std::collections::HashMap;
use std::fmt;
use std::io::{self, Error, Write};

use crate::dictionary::{ComputerDictionary, DictionaryLike, WebDictionary};

#[cfg(not(target_arch = "wasm32"))]
use crossterm::{
    cursor, queue, style,
    style::Color,
    terminal::{self, ClearType},
};

pub struct Board {
    word: String,
    word_count: HashMap<char, i32>,
    pub rows: [[Cell; 5]; 6],
    pub dictionary: Box<dyn DictionaryLike + Send>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Green(char),
    Yellow(char),
    Gray(char),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Cell::Gray(v) => write!(f, "{}", v),
            &Cell::Yellow(v) => write!(f, "{}", v),
            &Cell::Green(v) => write!(f, "{}", v),
            &Cell::Empty => write!(f, "{}", ""),
        }
    }
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
            dictionary: Box::new(ComputerDictionary::new("./data/dictionary.txt")),
        }
    }

    pub fn new_wasm(word: String) -> Self {
        let word_count = count_chars(&word);
        Board {
            word: word,
            word_count: word_count,
            rows: [[Cell::new(); 5]; 6],
            dictionary: Box::new(WebDictionary::new()),
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
        let index = self.rows.iter().position(|&r| r[0] == Cell::Empty).expect("You lose!");
        let guess = guess.trim();
        if guess.len() > 5 {
            panic!("can't guess more than 5")
        }
        self.word_count = count_chars(&self.word);
        self.mark_green(index, guess);
        self.mark_yellow(index, guess);
        self.mark_gray(index, guess);
    }

    pub fn has_won(&self) -> bool {
        // Check each row
        // Make sure every cell is green and has the right letter
        // Return true if any row matches that
        return self.rows.iter().any(|&r| {
            return r.iter().all(|c| {
                let is_green_cell = match c {
                    Cell::Green(_) => true,
                    _ => false,
                };
                return is_green_cell;
            });
        });
    }

    pub fn slack(&self) -> String {
        let mut response = "".to_string();
        for r in self.rows.iter() {
            for c in r {
                match c {
                    Cell::Green(value) | Cell::Yellow(value) | Cell::Gray(value) => {
                        response.push_str(&format!("   {}", value).to_uppercase());
                    }
                    Cell::Empty => {}
                }
            }
            response.push_str("\n");
            for c in r {
                match c {
                    Cell::Green(_) => {
                        response.push_str("🟩 ");
                    }
                    Cell::Yellow(_) => {
                        response.push_str("🟨 ");
                    }
                    Cell::Gray(_) => {
                        response.push_str("⬜️ ");
                    }
                    Cell::Empty => {
                        response.push_str("⬛️ ");
                    }
                }
            }
            response.push_str("\n");
        }
        return response;
    }

    // TODO: use a trait here instead
    #[cfg(not(target_arch = "wasm32"))]
    pub fn print(&self) -> Result<(), Error> {
        terminal::enable_raw_mode().expect("Failed to enter raw mode");
        queue!(
            io::stdout(),
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        // INSERT CODE HERE
        for (idx, r) in self.rows.iter().enumerate() {
            queue!(
                io::stdout(),
                style::ResetColor,
                cursor::Hide,
                cursor::MoveTo(0, idx.try_into().unwrap())
            )?;
            for c in r {
                match c {
                    Cell::Green(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Green),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        )?;
                    }
                    Cell::Yellow(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Yellow),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        )?;
                    }
                    Cell::Gray(value) => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::Grey),
                            style::SetForegroundColor(Color::Black),
                            style::Print(format!("  {}  ", value))
                        )?;
                    }
                    Cell::Empty => {
                        queue!(
                            io::stdout(),
                            style::SetBackgroundColor(Color::DarkGrey),
                            style::Print("     ")
                        )?;
                    }
                }
            }
        }
        queue!(io::stdout(), style::ResetColor).expect("Failed to reset color");

        // END CODE
        io::stdout().flush()?;
        terminal::disable_raw_mode()?;
        return Ok(());
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.rows {
            for c in r {
                write!(f, "{:?} | ", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::Cell;

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

    #[test]
    fn has_won() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rusty");
        assert_eq!(board.has_won(), true);
    }

    #[test]
    fn has_won_2nd() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rogue");
        assert_eq!(board.has_won(), false);
        board.guess("rusty");
        assert_eq!(board.has_won(), true);
    }

    #[test]
    fn has_won_multiple() {
        let mut board = Board::new("rusty".to_string());
        board.guess("rogue");
        assert_eq!(board.has_won(), false);
        board.guess("rusty");
        board.guess("rusty");
        assert_eq!(board.has_won(), true);
    }
}
