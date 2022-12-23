#[allow(dead_code, unused)]
mod routes;
mod db;
use db::connect_database;
use routes::*;
use actix_files as fs;
use fs::NamedFile;
use actix_web::{App,web, HttpServer, HttpRequest, Result};
use std::path::PathBuf;


/// https://actix.rs/docs/static-files/
async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = connect_database().await;
    let db = web::Data::new(db);
    HttpServer::new(move || 
        App::new()
        .app_data(db.clone())
        .service(fs::Files::new("/static", ".").show_files_listing())
        .route("/", web::get().to(index))
        .route("/store", web::post().to(store))
        // .service(store)
        .route("/get_messages", web::get().to(get_messages))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}