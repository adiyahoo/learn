

#[allow(dead_code)]
struct Car{ 
    brand: String, 
    model: String, 
    year: String 
}

impl Car{ 
    fn new(brand: String, model: String, year: String) -> Self {
        Car {
            brand,
            model,
            year,
        }
    }

    fn start_engine(&self){ 
        println!("{} Engine started!", self.brand);
    }
}

fn main(){ 
    let new_car = Car::new(String::from("Honda"), String::from("Civic"), String::from("2020"));
    new_car.start_engine();
}