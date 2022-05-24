use rpassword;
use std::io::{self, Write};

pub fn login() {
    print!("Login: ");
    io::stdout().flush().unwrap();

    let mut login_str = String::new();
    io::stdin()
        .read_line(&mut login_str)
        .expect("Failed to read line");
    let login = login_str.trim();

    let password = rpassword::prompt_password("Password: ").unwrap();

    println!("Test: {} {}", login, password);
}
