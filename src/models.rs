
use crate::schema::{books, authors, books_authors, self};
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, Error};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
type DbError = Box<dyn std::error::Error + Send + Sync>;




use super::DbPool;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Selectable, PartialEq, Debug, QueryableByName)]
pub struct Author {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
}


#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonMessage {
    pub message: String,
}


#[derive(Serialize, Deserialize, Queryable, Associations, Debug, PartialEq, Selectable, Identifiable)]
#[diesel(belongs_to(Author,  foreign_key = author_id))]
#[diesel(belongs_to(Book, foreign_key = book_id))]
pub struct BooksAuthor {
    pub id: i32,
    pub author_id: i32,
    pub book_id: i32
}


#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = books_authors)]
pub struct NewBookAuthor {
    pub author_id: i32,
    pub book_id: i32,
}

pub fn create_author(
    conn: &mut MysqlConnection,
    p_name: &str,
    
) -> Result<usize, DbError> {
    use crate::schema::authors::dsl::*;
    let new_author = NewAuthor {
        name: p_name.to_string(), 
    };
    let rows = diesel::insert_into(authors)
        .values(&new_author)
        .execute(conn)
        .expect("could not insert author");
    Ok(rows)
}


pub fn create_book(
    conn: &mut MysqlConnection,
    p_title: &str,
) -> Result<usize, DbError> {
    use crate::schema::books::dsl::*;
    let new_book = NewBook {
        title: p_title.to_string(),
    };

    let rows = diesel::insert_into(books)
        .values(&new_book)
        .execute(conn)
        .expect("could not insert book");
    Ok(rows)
}

pub fn create_books_author(conn: &mut MysqlConnection,
    p_author_id: i32, p_book_id: i32) ->Result<usize, DbError>{
        use crate::schema::books_authors::dsl::*;
        let new_book_author = NewBookAuthor{
            author_id: p_author_id,
            book_id: p_book_id,
        };

        let rows = diesel::insert_into(books_authors)
            .values(&new_book_author)
            .execute(conn)
            .expect("could not insert author and book");
        Ok(rows)
} 



pub fn get_authors_with_books(conn: &mut MysqlConnection) ->   Result<Vec<AuthorsWithBooks>, DbError> {

    /*let authors_with_book = authors::table
        .left_join(books_authors::table.on(books_authors::author_id.eq(authors::id)))
        .left_join(books::table.on(books_authors::book_id.eq(books::id)))
        .select((authors::all_columns, books::all_columns))
        .load::<(Author, Book)>(conn)?;
        println!("consulta: {authors_with_book:?}");

   
    Ok(())*/
    /* let query = authors::table
        .inner_join(books_authors::table.on(books_authors::author_id.eq(authors::id)))
        .left_join(books::table.on(books_authors::book_id.eq(books::id)))
        .select((authors::all_columns, books::all_columns.nullable()))
        .load::<(Author, Option<Book>)>(conn)?; */
        //.select((authors::all_columns, books::all_columns.nullable()))
        //.load::<(Author, Option<Book>)>(conn)?;

         /* let query = authors::table
        .left_join(books_authors::table.on(books_authors::author_id.eq(authors::id)))
        .left_join(books::table.on(books_authors::book_id.eq(books::id)))
        .select((authors::all_columns, books::all_columns.nullable()))
        .load::<(Author, Option<Book>)>(conn)?; */
     



        
        /* let query = authors::table
            .left_join(books_authors::table.on(authors::id.eq(books_authors::author_id)))
            .left_join(books::table.on(books_authors::book_id.eq(books::id)))
            .select((authors::all_columns, diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Text>>("GROUP_CONCAT(DISTINCT books.id) as book_ids")))
            .group_by(authors::id)
            .load::<(Author, Option<String>)>(conn)?;
    
        let authors_with_books = query.into_iter().map(|(author, book_ids)| {
            let books = book_ids.and_then(|ids| {
                let ids: Vec<i32> = ids.split(',').map(|id| id.parse().unwrap()).collect();
                let books_query = books::table.filter(books::id.eq_any(ids));
                let books_result = books_query.load::<Book>(conn).ok();
                books_result
            });
    
            AuthorsWithBooks2 { author, books }
        }).collect();
    
        Ok(authors_with_books)  */


     let all_authors = authors::table
        .select(Author::as_select())
        .load::<Author>(conn)
        .expect("Error al conectar");
    let books_all = BooksAuthor::belonging_to(&all_authors)
        .inner_join(books::table)
        .select((BooksAuthor::as_select(), Book::as_select()))
        .load::<(BooksAuthor, Book)>(conn)
        .expect("Error al cargar");

       let m=  books_all
            .grouped_by(&all_authors)
            .into_iter()
            .zip(all_authors)
            .map(|(a, author)|{
                (
                    author,
                    a.into_iter().map(|(_, books)| books).collect::<Vec<Book>>(),
                )
            })
            .collect::<Vec<(Author, Vec<Book>)>>()
            .into_iter()
            .map(|(author, books)| AuthorsWithBooks {author, books})
            .collect();

            Ok(m) 
    
}
            

    #[derive(Serialize, Queryable, Debug)]
    pub struct AuthorsWithBooks2 {
        pub author: Author,
        pub books: Option<Vec<Book>>,
    }
    
 impl AuthorsWithBooks2 {
    pub fn from_author_and_books(author: Author, books: Option<Vec<Book>>) -> Self {
        AuthorsWithBooks2 { author, books }
    }
} 


 #[derive(Serialize, Queryable)]
pub struct AuthorsWithBooks {
    author: Author,
    books: Vec<Book>,
} 

/* let items_with_photos: Vec<(Item, Vec<Photo>)> = items::table
.left_join(item_photos::table.on(item_photos::item_id.eq(items::id)))
.left_join(photos::table.on(item_photos::photo_id.eq(photos::id.nullable())))
.select((items::all_columns, photos::all_columns))
.load::<(Item, Option<Photo>)>(&connection)?
.grouped_by(|(item, _)| item.clone())
.into_iter()
.map(|(item, photos)| (item, photos.into_iter().filter_map(|(_, photo)| photo).collect()))
.collect(); */


pub fn get_authors_with_books3(conn: &mut MysqlConnection) ->   Result<Vec<AuthorsWithBooks2>, DbError> {

          let results = authors::table
        .load::<Author>(conn)?
        .into_iter()
        .map(|author| {
            let books = BooksAuthor::belonging_to(&author)
                .inner_join(books::table)
                .load::<(BooksAuthor, Book)>(conn)
                .map(|result| result.into_iter().map(|(_, book)| book).collect())
                .ok();
            AuthorsWithBooks2 { author, books }
        })
        .collect();

    Ok(results)  
}



pub fn get_authors_with_books2(conn: &mut MysqlConnection) -> Result<Vec<AuthorsWithBooks>, DbError>  {
    
    use crate::schema::authors::dsl::*;
    use crate::schema::books::dsl::*;
    use crate::schema::books_authors::dsl::*;

    
    let results = authors
        .inner_join(books_authors.inner_join(books))
        .load::<(Author, (BooksAuthor, Book))>(conn)
        .expect("Error");

        let mut authors_with_books: Vec<AuthorsWithBooks> = Vec::new();

        for (author, (_, book)) in results {
            if let Some(author_with_books) = authors_with_books
                .iter_mut()
                .find(|author_with_books| author_with_books.author.id == author.id)
            {
                author_with_books.books.push(book);
            } else {
                authors_with_books.push(AuthorsWithBooks {
                    author: author,
                    books: vec![book],
                });
            }
        }
    
        Ok(authors_with_books)
}



#[get("/")]
async fn get_authors(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    
    let res = web::block(move || {
        let mut conn = pool.get()?;
        get_authors_with_books(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}


#[get("/v2")]
async fn get_authors2(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    
    let res = web::block(move || {
        let mut conn = pool.get()?;
        get_authors_with_books2(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/v3")]
async fn get_authors3(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    
    let res = web::block(move || {
        let mut conn = pool.get()?;
        get_authors_with_books3(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}



#[post("/")]
async fn post_author(
    pool: web::Data<DbPool>,
    form: web::Json<NewAuthor>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        create_author(&mut conn, &form.name)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let res = JsonMessage {
        message: "New author created!".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}


#[post("/")]
async fn post_book(
    pool: web::Data<DbPool>,
    form: web::Json<NewBook>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        create_book(&mut conn, &form.title)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let res = JsonMessage {
        message: "New Book created!".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}


#[post("/{author_id}/{id_book}")]
pub async fn post_books_authors(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>) ->Result<HttpResponse, Error>{

        let(author_id,book_id, ) = path.into_inner();
        web::block(move || {
            let mut conn = pool.get()?;
            create_books_author(&mut conn, author_id, book_id)
        })
        .await?
        .map_err(ErrorInternalServerError)?;
    
        let res = JsonMessage {
            message: "New author and book created!".to_string(),
        };

        Ok(HttpResponse::Ok().json(res))
    
    }




   

 
