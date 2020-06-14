# Webserver


```rust
[dependencies]
actix-rt = "1.1.1"
actix-web = "2.0.0"
```

src/main.rs
```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
```