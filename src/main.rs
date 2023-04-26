#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::content::Html;
use rocket::response::NamedFile;
use std::path::PathBuf;

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    let path: PathBuf = "./static/arch_linux.ico".into();
    NamedFile::open(&path).ok()
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

fn main() {
    rocket::ignite()
            .mount("/", routes![index])
            .mount("/", routes![favicon])
            .launch();
}
