use std::fmt::Display;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Person: {} ({})", self.name, self.age)
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_available: bool
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let availability = if self.is_available { "available" } else { "not available" };
        write!(f,"Book: {} by {} ({})", self.title, self.author, availability)
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut books_str = "".to_string();

        for book in &self.books {
            books_str.push_str(&format!("{}\n", book));
        }
        write!(f,"Library:\n{}", books_str)
    }
}


fn main() {
    let person = Person { 
        name: "Akhil".to_string(),
        age: 20
    };

    let library = Library {
        books: vec![
            Book {
                title: "Harry Potter".to_string(),
                author: "J.K. Rowling".to_string(),
                is_available: true
            }, 
            Book {
                title: "Designing Data Intensive Applications".to_string(),
                author: "Martin Kleppmann".to_string(),
                is_available: false
            }
        ]
    };

    println!("{}", person);
    println!("{}", library);
}
