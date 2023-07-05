use regex::Regex;

fn main() { 
   let re = Regex::new(r"(\w{5})").unwrap();
   let text = "rhani"; 

   match re.captures(text) { 
    Some(caps) => println!("ada value: {}", caps.get(0).unwrap().as_str()),
    None => println!("could not find match...")
   }
}