use actix_web::{get, web, App, HttpServer, Responder};

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db_connection;
pub mod models;
pub mod schema;

pub mod handlers; // This goes to the top to load the next handlers module

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/products").route(web::get().to(handlers::products::index)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
