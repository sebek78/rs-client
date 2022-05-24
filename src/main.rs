mod api;
mod auth;
use crate::api::get_data;
use crate::auth::login;
use std::io;

#[tokio::main]
async fn main() {
    loop {
        // TODO: use crossterm = "0.23"
        println!("Commands: [ quit, login, get ]");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let cmd = command.trim();

        match cmd {
            "quit" => break,
            "login" => login(),
            "get" => get_data().await,
            _ => println!("Unknown command: {}", command),
        }
    }
}
