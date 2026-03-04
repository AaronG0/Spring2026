use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    
    for book in books{
        writeln!(file, "{} by {}, {}", book.title, book.author, book.year);
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let mut booksvec: Vec<Book> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();  // panic if error

        if line.trim().is_empty() {
            continue;
        }
        booksvec.push(parse_book_line(&line));

        //if let Some(book) = parse_book_line(&line) {
       //     books.push(book);
     //   }
    }
    return booksvec;
}

fn parse_book_line(line: &str) -> Book {
    let (title_part, rest) = line.split_once(" by ").unwrap();
    let (author_part, year_part) = rest.split_once(", ").unwrap();
    let year: u16 = year_part.parse().unwrap();

    Book {
        title: title_part.to_string(),
        author: author_part.to_string(),
        year,
    }
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}