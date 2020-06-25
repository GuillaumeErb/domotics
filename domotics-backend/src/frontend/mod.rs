use rocket::response::NamedFile;
use std::{
    io,
    path::{Path, PathBuf},
};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[get("/www/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}
