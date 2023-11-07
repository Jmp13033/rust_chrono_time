#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*; 
use diesel::pg::PgConnection; 
use std::env;
use dotenvy::dotenv;
mod schema;
mod models;
use models::{Product};  
 
use serde_json::json;
use crate::schema::product::dsl::product;
use rocket_contrib::json::{Json, JsonValue};
use rocket::{get, put, post, delete, routes,}; 



// connection to postgres 
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[get("/product")]
pub fn get_product() -> Json<JsonValue> {

    let connection = establish_connection();

    let get_products = product.load::<Product>(&connection)
    .expect("Error loading user");

    Json(JsonValue::from(json!({
        "users": get_products,
    })))


}



fn main() {
    rocket::ignite().mount("/", routes![
        get_product
]).launch();
}
