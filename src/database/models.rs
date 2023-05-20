
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use super::schema::vocabulary;


#[derive(Insertable, Deserialize)]
#[diesel(table_name = vocabulary)]
pub struct NewVocaItem {
    pub en: String,
    pub tr: String
}

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = vocabulary)]
pub struct VocaItem {
    pub id: i32,
    pub en: String,
    pub tr: String
}