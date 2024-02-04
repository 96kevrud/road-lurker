#[macro_use] extern crate rocket;

use rocket::{fs::NamedFile, response::Redirect};
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/index.html"))
}

#[get("/index.html")]
async fn home () -> Option<NamedFile> {
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
