use reqwest::header::{CONTENT_TYPE, ACCEPT, AUTHORIZATION};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/")
        .header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
