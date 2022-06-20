mod api;
mod auth;
use crate::api::get_data;
use crate::auth::{login, logout};
use reqwest::Client;
use std::io;

#[tokio::main]
async fn main() {
    let client = Client::builder().cookie_store(true).build().unwrap();

    loop {
        println!("Commands: [ quit, login, get, logout ]");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let cmd = command.trim();

        match cmd {
            "quit" => break,
            "login" => login(&client).await,
            "get" => get_data(&client).await,
            "logout" => logout(&client).await,
            _ => println!("Unknown command: {}", command),
        }
    }
}
