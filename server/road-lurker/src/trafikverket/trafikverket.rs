
use reqwest;
use regex::Regex;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use futures::future::join_all;
use reqwest::header::CONTENT_TYPE;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::{fs::File, io::{copy, Cursor}};
use sha256::try_digest;

pub struct DataFetcher{}

impl DataFetcher {

    pub fn create() -> Self {
        DataFetcher {}
    }

    pub async fn start(&self){
        loop {
            let cameras = fetch_cameras().await;
            fs::write("data/cameras.json", &cameras.to_string())
                .expect("Could not write to file");
            fetch_images(cameras).await;
            sleep(Duration::from_secs(10))
        }
    }
}

async fn fetch_images(camera_json: serde_json::Value){
    let cameras = camera_json.get("cameras").unwrap();

    let mut futures = vec![];
    for cam in cameras.as_array().unwrap(){
        let url = cam.get("url").unwrap().as_str().unwrap();
        let future = fetch_image(&cam["id"].as_str().unwrap(), url);
        futures.push(future)
    }
    join_all(futures).await;

}


async fn fetch_image(id: &str, url: &str){     
    
    let image = reqwest::get(url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let image_dir = format!("data/images/{}", id);        
    fs::create_dir_all(&image_dir).expect("Could not create dirs");
    let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let image_path = format!("{}/{}.jpeg", image_dir, time_now);
    let mut file = File::create(&image_path).unwrap();
    let mut content =  Cursor::new(image);
    copy(&mut content, &mut file).ok();

    let hash = try_digest(Path::new(&image_path)).unwrap();
    let hash_path = format!("{}/{}.txt", image_dir, "sha256sum");
    let mut hash_file = File::create(&hash_path).unwrap();
    let mut hash_content =  Cursor::new(hash);
    copy(&mut hash_content, &mut hash_file).ok();
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
        
        let x = camera["Id"].to_string();
        let id = &x[1..x.len()-1];
        new_cams.push(json!({
            "id": id,
            "active": camera["Active"],
            "longitude": &coords[1].parse::<f32>().unwrap(),
            "latitude": &coords[2].parse::<f32>().unwrap(),
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
