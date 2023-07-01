use std::env; 

fn main() { 

    let args: Vec<String> = env::args().collect();
    let buffer = String::new(); 

   println!("{} {}", args[1], args[2]);

}