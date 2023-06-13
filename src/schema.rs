// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
    }
}

diesel::table! {
    books_authors (id) {
        id -> Integer,
        author_id -> Integer,
        book_id -> Integer,
    }
}

diesel::table! {
    phone (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    prueba (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(books_authors -> authors (author_id));
diesel::joinable!(books_authors -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors,
    phone,
    prueba,
);
