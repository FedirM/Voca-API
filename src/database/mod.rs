

mod models;
mod schema;
mod db;

pub use models::{NewVocaItem, VocaItem};

use db::connect;
use schema::vocabulary::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn add( item: NewVocaItem ) -> Result<usize, Error>{
    
    let con = &mut connect();
    diesel::insert_into(vocabulary)
        .values(&item)
        .execute(con)?;

    Ok(1)
}

pub fn update( item: VocaItem ) -> Result<usize, Error> {
    let con = &mut connect();
    diesel::update( vocabulary.find(item.id) )
        .set(&item)
        .execute(con)?;
    Ok(1)
}

pub fn get( word: &str ) -> Result<Vec<VocaItem>, Error> {
    let con = &mut connect();
    let res = vocabulary
        .filter(en.eq(word))
        .load::<VocaItem>(con)?;
    Ok(res)
}


