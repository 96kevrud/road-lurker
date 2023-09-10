
use reqwest;
use std::env;

pub async fn test() -> String {
    let body = reqwest::get("https://www.rust-lang.org").await.unwrap().text().await.unwrap();
    println!("{}", get_apikey());
    body
}

fn get_apikey() -> String {
    let env_var = "TRAFIKVERKET_APIKEY";
    let value = match env::var(env_var) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", env_var, e)
    };
    println!("{}: {}", env_var, value);
    return value
}