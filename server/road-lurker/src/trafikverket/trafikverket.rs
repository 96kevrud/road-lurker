
use reqwest;
use std::env;
use std::fs;
use reqwest::header::CONTENT_TYPE;

pub async fn fetch_cameras(){
    let url = "https://api.trafikinfo.trafikverket.se/v2/data.json";

    let api_key = get_apikey();
    let query = fs::read_to_string("src/trafikverket/xml/post.xml")
        .expect("Could not find trafikverket post query file!")
        .replace("APIKEY", &api_key);

    let client = reqwest::Client::new();

    let res = client.post(url)
        .body(query)
        .header(CONTENT_TYPE, "text/xml")
        .send()
        .await
        .unwrap();

    let body = res.text().await.unwrap();

    fs::write("src/trafikverket/data/cameras.json", body)
        .expect("Could not write to file")
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
