mod book;
mod library;
mod person;

use library::Library;
use person::{display_person_name_and_age, Person};

fn main() {
    let person = Person {
        name: "Akhil".to_string(),
        age: 20,
    };

    display_person_name_and_age(&person); // Akhil (20)

    let mut library = Library::create_library();

    library.add_book("Harry Potter", "J.K. Rowling");
    library.add_book("Design Patterns", "Erich Gamma");
    library.add_book("The Pragmatic Programmer", "Andy Hunt");
    library.add_book("Designing Data Intesive Applications", "Martin Kleppmann");
    library.add_book("The Rust Programming Language", "Steve Klabnik");
    library.add_book("Distributed Systems", "Maarten van Steen");

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
