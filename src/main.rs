use reqwest;
use std::io;

async fn get_data() {
    let result = reqwest::get("http://localhost:3333/api")
        .await
        .unwrap()
        .text()
        .await;

    println!("{:?}", result);
}

#[tokio::main]
async fn main() {
    println!("Commands: [ quit, get ]");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    let cmd = command.trim();

    match cmd {
        "quit" => println!("Q"),
        "get" => get_data().await,
        _ => println!("Unknown command: {}", command),
    }
}
