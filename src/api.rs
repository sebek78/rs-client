use reqwest::Client;

pub async fn get_data(client: &Client) {
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
