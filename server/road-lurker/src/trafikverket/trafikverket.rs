
use reqwest;


pub async fn test() -> String {
    let body = reqwest::get("https://www.rust-lang.org").await.unwrap().text().await.unwrap();
    body
}