use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};
mod db;
mod routes;
mod server;
mod session;
use actix::*;
use actix_files as fs;
use actix_web::{ web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web_actors::ws;
use db::connect_database;
use fs::NamedFile;
use routes::*;
use std::path::PathBuf;

/// https://actix.rs/docs/static-files/
async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    println!("Visitors: {current_count}");
    format!("Visitors: {current_count}")
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();


    let db = connect_database().await;
    let db = web::Data::new(db);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/store", web::post().to(store))
            .route("/ws", web::get().to(chat_route))
            .route("/count", web::get().to(get_count))
            .route("/get_messages", web::get().to(get_messages))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
