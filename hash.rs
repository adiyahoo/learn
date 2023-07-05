use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    
    match marks.get("Rust Programming") {
        Some(mark) => println!("Dapat kode rust {}", mark),
        None => println!("Dapat kontol")
    }

    marks.remove("UX Design");

    for (subject, id) in &marks {
        println!("subject: {}, id: {}", subject, id);
    }
}