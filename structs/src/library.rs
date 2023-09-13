use std::{fmt::Display, io::Error, io::ErrorKind};

use crate::book::Book;

#[derive(Debug)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn create_library() -> Library {
        Library { books: vec![] }
    }

    pub fn add_book(&mut self, title: &str, author: &str) {
        let book = Book::create_book(title, author);

        self.books.push(book);
    }

    pub fn checkout_book(&mut self, book_name: &str, borrower: &str) -> Result<&Book, Error> {
        let book = match self.books.iter_mut().find(|b| b.title == book_name) {
            Some(book) => book,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Book {} not found in library", book_name),
                ))
            }
        };

        if book.is_borrowed {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Book {} was already borrowed", book_name),
            ));
        }

        book.is_borrowed = true;
        book.borrower = Some(borrower.to_string());

        Ok(book)
    }

    pub fn return_book(&mut self, book_name: &str) -> Result<&Book, Error> {
        let book = match self.books.iter_mut().find(|b| b.title == book_name) {
            Some(book) => book,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Book {} not found in library", book_name),
                ))
            }
        };

        if !book.is_borrowed {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Book {} was not borrowed", book_name),
            ));
        }

        book.is_borrowed = false;
        book.borrower = None;

        Ok(book)
    }

    pub fn list_books(&self) {
        self.books.iter().for_each(|book| println!("{}", book));
    }

    pub fn list_checked_out_books(&self) {
        self.books
            .iter()
            .filter(|book| book.is_borrowed)
            .for_each(|book| {
                println!("{}", book);
            });
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut books_str = "".to_string();

        for book in &self.books {
            books_str.push_str(&format!("{}\n", book));
        }
        write!(f, "Library:\n{}", books_str)
    }
}
