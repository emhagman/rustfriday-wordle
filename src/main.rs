mod board;
mod dictionary;
mod slack;
mod utils;

#[macro_use]
extern crate lazy_static;

use crate::board::Board;
use httpserver::HttpServer;
use serde_json::Value;
use std::{
    io::{self, Error, Write},
    sync::Mutex,
};

lazy_static! {
    static ref BOARD: Mutex<Board> = Mutex::new(Board::new("rusty".to_string()));
}

fn slack() {
    let mut server = HttpServer::new();
    server.get("/", &|req| {
        return "health_check".to_string();
    });
    server.post("/events", &|req| {
        let mut board = BOARD.lock().unwrap();

        // parse the json body
        let raw_body = req.body.as_ref().unwrap();
        let v: Value = serde_json::from_str(&raw_body).expect("Can't parse JSON");

        // don't respond if the bot posted the message
        if v["event"]["user"] == "U03M2JWUMDY" {
            return "NO_MESSAGE".to_string();
        }

        // response to text
        let input = v["event"]["text"].to_string();
        let trimmed_input = input.trim_matches('"');
        if input != "" {
            if input == "null" {
                return "".to_string();
            }
            if !board.dictionary.is_a_word(trimmed_input) {
                slack::send_slack_message_to_channel(
                    "rust-wordle-bot",
                    &format!("{} is not in the dictionary!", trimmed_input.trim()),
                );
            } else {
                board.guess(trimmed_input);
                board.print().expect("Failed to print board to terminal");
                slack::send_slack_message_to_channel("rust-wordle-bot", &board.slack());
            }
            // TODO: add checks for type of command here
        }
        // return the challenge response if needed
        return v["challenge"].to_string();
    });
    server.listen();
}

fn terminal() -> Result<(), Error> {
    loop {
        let mut input = String::new();
        print!("\nMake a guess: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input).expect("failed to read guess");
        let mut board = BOARD.lock().unwrap();
        if !board.dictionary.is_a_word(&input) {
            println!("{} is not in the dictionary!", &input.trim());
        } else {
            board.guess(&input);
            board.print()?;
        }
    }
}

fn main() {
    // slack();
    // terminal().expect("Failed to run terminal loop");
}
