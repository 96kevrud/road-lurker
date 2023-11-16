#[macro_use] extern crate rocket;

//mod trafikverket;

use rocket::{fs::NamedFile, response::Redirect};
use std::path::{Path, PathBuf};
use trafikverket::sync_cameras;

#[get("/")]
fn index() -> Redirect {
    println!("index");
    Redirect::to(uri!("/index.html"))
}

#[get("/index.html")]
async fn home () -> Option<NamedFile> {
    println!("inside home");
    sync_cameras().await;
    NamedFile::open("client/index.html").await.ok()
}

#[get("/data/<file..>")]
async fn data_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("data").join(file)).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, home, data_files, files])
}
