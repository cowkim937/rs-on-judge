mod common;

use common::app::{index, echo, manual_hello, AppState};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}