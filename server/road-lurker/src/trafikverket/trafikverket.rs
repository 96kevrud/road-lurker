
use reqwest;
use regex::Regex;
use serde_json::json;
use std::env;
use std::fs;
use reqwest::header::CONTENT_TYPE;

pub async fn sync_cameras(){
    let cameras = fetch_cameras().await;

    fs::write("data/cameras.json", &cameras.to_string())
        .expect("Could not write to file")
}

async fn fetch_cameras() -> serde_json::Value {
    let cameras = query_api_cameras().await;
    return reformat_camera_data(cameras)
}

async fn query_api_cameras() -> serde_json::Value{
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

    let body = res.json::<serde_json::Value>().await.unwrap();

    return body["RESPONSE"]["RESULT"][0].to_owned();
}

fn reformat_camera_data(camera_json: serde_json::Value) -> serde_json::Value {
    let cameras = camera_json["Camera"].as_array().unwrap();

    let mut new_cams: Vec<serde_json::Value> = Vec::new();
    for camera in cameras {
        let point: String = camera["Geometry"]["WGS84"].to_string();
        let re: Regex = Regex::new(r"POINT \(([0-9]+\.[0-9]+) ([0-9]+\.[0-9]+)\)").unwrap();
        let coords = re.captures(&point).expect("Could not match coords");

        let direction = match camera["Direction"].to_string().parse::<i32>() {
            Ok(v) => v,
            Err(_) => 999
        };

        new_cams.push(json!({
            "active": camera["Active"],
            "latitude": &coords[1].parse::<f32>().unwrap(),
            "longitude": &coords[2].parse::<f32>().unwrap(),
            "direction": direction,
            "phototime": camera["PhotoTime"],
            "url": camera["PhotoUrl"]
        }));
    }

    return json!({
        "cameras" : new_cams
    });
}

fn get_apikey() -> String {
    let env_var = "TRAFIKVERKET_APIKEY";
    let value = match env::var(env_var) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", env_var, e)
    };
    return value
}
