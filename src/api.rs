use reqwest;

pub async fn get_data() {
    let result = reqwest::get("http://localhost:3333/api")
        .await
        .unwrap()
        .text()
        .await;

    println!("{:?}", result);
}
