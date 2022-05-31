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

pub async fn login() {
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

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = client
        .post("http://localhost:3333/api/auth/login")
        .json(&data)
        .send()
        .await
        .expect("send login data");

    //let cookies = response.headers().get_all("set-cookie");
    //let mut iter = cookies.iter();
    // println!("Cookie Authentication {:?}", iter.next());
    // println!("Cookie Refresh {:?}", iter.next());

    let user_dto = response.json::<UserDto>().await.unwrap();
    println!("{:#?}", user_dto.user.username);

    let result = client
        .get("http://localhost:3333/api")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", result);
}
