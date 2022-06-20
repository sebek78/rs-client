use reqwest::Client;
use rpassword;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct LoginDto {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    role: String, // enum USER | ADMIN
}

#[derive(Debug, Serialize, Deserialize)]
struct UserDto {
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogoutDto {
    success: bool,
}

pub async fn login(client: &Client) {
    print!("Login: ");
    io::stdout().flush().unwrap();

    let mut login_str = String::new();
    io::stdin()
        .read_line(&mut login_str)
        .expect("Failed to read line");
    let login = login_str.trim();

    let password = rpassword::prompt_password("Password: ").unwrap();

    let data = LoginDto {
        username: login.into(),
        password: password.into(),
    };

    let response = client
        .post("http://localhost:3333/api/auth/login")
        .json(&data)
        .send()
        .await
        .expect("send login data");

    let user_dto = response.json::<UserDto>().await.unwrap();
    println!("{:#?}", user_dto.user.username);
}

pub async fn logout(client: &Client) {
    let response = client
        .post("http://localhost:3333/api/auth/logout")
        .send()
        .await
        .expect("send login data");

    let data_dto = response.json::<LogoutDto>().await.unwrap();
    println!("{:#?}", data_dto.success);
}
