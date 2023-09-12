use std::{fmt::Display, io::Error, io::ErrorKind};

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person: {} ({})", self.name, self.age)
    }
}

fn display_person_name_and_age(person: &Person) {
    println!("{} ({})", person.name, person.age);
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
    borrower: Option<String>,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut availability = if self.is_borrowed {
            "Not available".to_string()
        } else {
            "Available".to_string()
        };

        match self.borrower {
            Some(ref name) => availability.push_str(&format!("[Borrowed by {}])", name)),
            None => (),
        }

        write!(
            f,
            "Book: {} by {} ({})",
            self.title, self.author, availability
        )
    }
}

impl Book {
    fn create_book(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
            borrower: None,
        }
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn checkout_book(&mut self, book_name: &str, borrower: &str) -> Result<&Book, Error> {
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

    fn return_book(&mut self, book_name: &str) -> Result<&Book, Error> {
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

    fn list_books(&self) {
        self.books.iter().for_each(|book| println!("{}", book));
    }

    fn list_checked_out_books(&self) {
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

fn main() {
    let person = Person {
        name: "Akhil".to_string(),
        age: 20,
    };

    let mut library = Library {
        books: vec![
            Book::create_book("Harry Potter", "J.K. Rowling"),
            Book::create_book("Design Patterns", "Erich Gamma"),
            Book::create_book("The Pragmatic Programmer", "Andy Hunt"),
            Book::create_book("Designing Data Intesive Applications", "Martin Kleppmann"),
            Book::create_book("The Rust Programming Language", "Steve Klabnik"),
            Book::create_book("Distributed Systems", "Maarten van Steen"),
        ],
    };

    display_person_name_and_age(&person); // Akhil (20)

    println!("Books in library are: ");
    library.list_books();

    match library.checkout_book("Harry Potter", "Akhil") {
        Ok(book) => println!(
            "Checked out book: {} by {}",
            book.title,
            book.borrower.as_ref().unwrap()
        ),
        Err(err) => println!("Error: {}", err),
    }; // Checked out book: Harry Potter by Akhil

    match library.checkout_book("Design Patterns", "Vipul") {
        Ok(book) => println!(
            "Checked out book: {} by {}",
            book.title,
            book.borrower.as_ref().unwrap()
        ),
        Err(err) => println!("Error: {}", err),
    } // Checked out book: Design Patterns by Vipul

    println!("Checked out books are: ");
    library.list_checked_out_books();

    match library.checkout_book("Harry Potter", "Akhil") {
        Ok(book) => println!("Checked out book: {}", book),
        Err(err) => println!("Error: {}", err),
    }; // Error: Book Harry Potter was already borrowed

    match library.return_book("Harry Potter") {
        Ok(book) => println!("Returned book: {}", book),
        Err(err) => println!("Error: {}", err),
    }; // Returned book: Book: Harry Potter by J.K. Rowling (Available)

    match library.return_book("Harry Potter") {
        Ok(book) => println!("Returned book: {}", book),
        Err(err) => println!("Error: {}", err), // Error: Book Harry Potter was not borrowed
    };
}
