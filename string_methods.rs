fn main() { 
    { 
        let test = String::from("halo aku jamal"); 
        println!("Before Replace: {}", test);
        println!("After Replace: {}", test.replace("jamal", "adi"));
    }
}