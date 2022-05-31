mod api;
mod auth;
use crate::api::get_data;
use crate::auth::login;
use std::io;

#[tokio::main]
async fn main() {
    loop {
        println!("Commands: [ quit, login, get ]");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let cmd = command.trim();

        match cmd {
            "quit" => break,
            "login" => login().await,
            "get" => get_data().await,
            _ => println!("Unknown command: {}", command),
        }
    }
}
