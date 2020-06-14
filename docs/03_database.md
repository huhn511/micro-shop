# Database
# Database


```rust
[dependencies]
diesel = { version = "1.4.5", features = ["postgres"] }
dotenv = "0.15.0"
```


Run a Postgresql Database in a Docker container in deamon mode (-d).
Deamon mode means, it runs in the backround and you can use the console.

```bash
docker-compose up -d
```

Run this command to generate migration files
```bash
diesel migration generate create_products
```

up.sql
```rust
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  stock FLOAT NOT NULL,
  price INTEGER --representing cents
)
```

down.sql
```rust
DROP TABLE products
```


run the migration
```rust
diesel migration run
```


add this to the `src/main.rs`
```rust
#[macro_use]
extern crate diesel;
extern crate dotenv;
```


create a new file `src/db_connection.rs`, which handles the db connection
```rust
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Load the configuration in a postgres connection, 
    // the ampersand(&) means we're taking a reference for the variable. 
    // The function you need to call will tell you if you have to pass a
    // reference or a value, borrow it or not.
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
```


create a folder for all `models`:´with the file `products.rs`

src/models/product.rs
```rust
use crate::schema::products;

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32> // For a value that can be null, 
                           // in Rust is an Option type that 
                           // will be None when the db value is null
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}
```

 `src/models/mod.rs``
```rust
pub mod product;
```


make them available in `main.rs`
```rust
pub mod schema;
pub mod models;
pub mod db_connection;
```

add dependencies for json serializer
```rust
[dependencies]
serde = "1.0.111"
serde_derive = "1.0.111"
serde_json = "1.0.55"
```


now we can add the `list` method on our product model.
models/product.rs
```rust
// This will tell the compiler that the struct will be serialized and 
// deserialized, we need to install serde to make it work.
#[derive(Serialize, Deserialize)] 
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        // These four statements can be placed in the top, or here, your call.
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::products::dsl::*;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let result = 
            products
                .limit(10)
                .load::<Product>(&connection)
                .expect("Error loading products");

        // We return a value by leaving it without a comma
        ProductList(result)
    }
}
```

and add the Serialize and Deserialize makro to our product struct.

```rust
#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>
}
```

new we create a controller for the product endpoints.

create a new folder `controllers` and a `mod.rs` file

```bash
pub mod products;
```

src/handlers/products.rs:
```rust
use actix_web::{HttpRequest, HttpResponse };

use crate::models::product::ProductList;

// This is calling the list method on ProductList and 
// serializing it to a json response
pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ProductList::list())
}
```


src/main.rs:

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db_connection;
pub mod models;
pub mod schema;

pub mod controllers; // This goes to the top to load the next controllers module

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
```

