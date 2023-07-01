#[allow(dead_code)]
enum Animal { 
    Cat,
    Dog, 
    Bird
}

fn animal_method(animal: Animal){ 
    match animal { 
        Animal::Cat => println!("Hai ini kucing"),
        Animal::Dog => println!("Ini anjing"),
        Animal::Bird => println!("Ini burung"),
    };
}

fn main() {
    let animal: Animal = Animal::Cat; 
    animal_method(animal);
}