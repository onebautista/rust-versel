CREATE TABLE books_authors (
    id int PRIMARY KEY auto_increment,
    author_id int NOT NULL,
    book_id int NOT NULL,
    foreign key (author_id) references authors(id),
    foreign key (book_id) references books(id)
);