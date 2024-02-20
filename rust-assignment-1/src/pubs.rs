pub struct Book {
    title: String,
    author: String,
    page_count: u16,
}

pub struct Magazine {
    title: String,
    issue: u8,
    topic: String,
}

pub enum Publication {
    Book(Book),
    Magazine(Magazine),
}

impl Publication {
    pub fn view(&self) {
        match self {
            Publication::Book(ref book) => {
                println!(
                    "Book: {}, author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "Magazine: {}, issue: {}, topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                )
            }
        }
    }
}

pub fn dummy_pubs() -> Vec<Publication> {
    vec![
        Publication::Book(Book {
            title: "The Name of the Wind".to_string(),
            author: "Patrick Rothfuss".to_string(),
            page_count: 662,
        }),
        Publication::Magazine(Magazine {
            title: "Computing in Science & Engineering".to_string(),
            issue: 3,
            topic: "IEEE Transactions on Sustainable Computing".to_string(),
        }),
        Publication::Book(Book {
            title: "The Rust Programming Language".to_string(),
            author: "K. Rustacean".to_string(),
            page_count: 320,
        }),
        Publication::Magazine(Magazine {
            title: "Rust World".to_string(),
            issue: 10,
            topic: "Systems Programming".to_string(),
        }),
    ]
}
