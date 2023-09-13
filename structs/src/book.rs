use std::fmt::Display;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
    pub borrower: Option<String>,
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
    pub fn create_book(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
            borrower: None,
        }
    }
}
