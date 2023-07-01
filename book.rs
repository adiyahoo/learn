struct Book { 
    title: String, 
    author: String, 
    publication_year: u32  
} 

#[allow(dead_code)]
enum Genre { 
    Fiction,
    NonFiction,
    ScienceFiction
}

impl Book { 

    fn new(title: String, author: String, publication_year: u32) -> Self { 
        Book { 
            title,
            author, 
            publication_year
        }
    }

    fn info(&self, genre: Genre) {   
        let str_genre = match genre {
            Genre::Fiction => "Fiction",
            Genre::NonFiction => "NonFiction",
            Genre::ScienceFiction => "ScienceFiction"
        };

        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Publication_year: {}", self.publication_year);
        println!("Genre: {}", str_genre);
    }

}

fn main() { 

    let in_book = Book::new(
        String::from("Aku Sayang Kamu"), 
        String::from("Adiyaksa"),
        2020 // Year
    );

    let genre: Genre  = Genre::NonFiction;

    in_book.info(genre);
    
}