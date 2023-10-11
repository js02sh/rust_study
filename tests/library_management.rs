//cargo test --test library_management -- --nocapture
//use std::io;
#[derive(Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

struct Patron {
    name: String,
    books_checked_out: Vec<Book>,
}

impl Book {
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_avaliable: true,
        }
    }

    fn borrow(&mut self) {
        if self.is_avaliable {
            self.is_avaliable = false;
            println!("Book '{}' by {} has been borrowed.", self.title, self.author);
        } else {
            println!("Sorry, '{}' by {} is not available for borrowing.", self.title, self.author);
        }

    }

    fn return_book(&mut self) {
        if !self.is_avaliable {
            self.is_avaliable = true;
            println!("Book '{}' by {} has been returned.", self.title, self.author);
        }
    }
}

impl Patron {
    fn new(name: &str) -> Patron {
        Patron {
            name: name.to_string(),
            books_checked_out: Vec::new(),
        }
    }

    fn borrow_book(&mut self, book: &mut Book) {
        if book.is_avaliable {
            book.borrow();
            self.books_checked_out.push(book.clone());
            println!("{} has been borrowed '{}'", self.name, book.title);
        } else {
            println!("Sorry, '{}' by {} is not available for borrowing.", book.title, book.author);
        }
    }

    fn return_book(&mut self, book: &mut Book) {
        if self.books_checked_out.contains(book) {
            book.return_book();
            self.books_checked_out.retain(|b| b != book);
            println!("{} has returned '{}'", self.name, book.title);
        } else {
            println!("{} did not borrow '{}' from this libarary.", self.name, book.title);
        }
    }
}


fn main() {
    println!("Welcome to the Library Manager");

    let mut book1 = Book::new("Harry Potter", "J.K. Rolling");
    let mut patron1 = Patron::new("Jisang");
    
    patron1.borrow_book(&mut book1);
    patron1.borrow_book(&mut book1);

    // Borrow a different book and return the first book
    let mut book2 = Book::new("The Hobbit", "J.R.R Tolkien");
    patron1.borrow_book(&mut book2);
    patron1.return_book(&mut book1);
}


#[test]
fn test() {
    main();
}