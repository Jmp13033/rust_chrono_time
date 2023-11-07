use diesel::{Queryable, Insertable, AsChangeset}; 
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime; 
use diesel::prelude::*;
use super::schema::product; 



// orders struct 
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String, 
    pub created_at: Option<NaiveDateTime>
}



// users
#[derive(Insertable, Queryable, Deserialize)]
#[table_name = "product"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub created_at: Option<NaiveDateTime>,
    
}





