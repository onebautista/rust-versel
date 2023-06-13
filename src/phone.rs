
use crate::schema::{phone, self};
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, Error};
use actix_web::{web, HttpResponse};
use diesel::{prelude::*, sql_query};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
type DbError = Box<dyn std::error::Error + Send + Sync>;


use super::DbPool;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Selectable, PartialEq, Debug, QueryableByName)]
#[diesel(table_name = phone)]
pub struct Phone {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Test(String, String);
 /*  pub id: i32,
  pub name: String
} */


pub fn get_phones(conn: &mut MysqlConnection) -> Result<Vec<Phone>, DbError> {
//pub fn get_phones(conn: &mut MysqlConnection) -> Result<Test, DbError>  {
    let query = format!("SELECT * FROM phone");
    let resutl = sql_query(query).load::<Phone>(conn).unwrap();
    Ok(resutl) 
} 


#[get("/phone")]
async fn get_phones_query(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    
    let res = web::block(move || {
        let mut conn = pool.get()?;
        get_phones(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}