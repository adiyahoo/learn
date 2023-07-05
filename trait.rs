trait Drawable {
    fn draw(&self) -> String;
    fn get_area(&self) -> f64;
}

struct Rectangle { 
    width: f64,
    height: f64
}

struct Circle { 
    width: f64,
    height: f64
}

impl Drawable for Rectangle  {
    fn draw(&self) -> String {
       "Drawing a rectangle...".to_string()
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Circle { 
    fn draw(&self) -> String {
       "Drawing a circle...".to_string()
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_drawable_info(item: &impl Drawable) { 
    println!("{}", item.draw());
    println!("Area: {}", item.get_area());
}

fn main() {
    let rectangle = Rectangle { width: 50.0, height: 150.0 };
    let circle = Circle { width: 80.0, height: 100.0 };

    print_drawable_info(&rectangle);
    print_drawable_info(&circle);

    //rect
    // println!("{}", rectangle.draw());
    // println!("Area: {}", rectangle.get_area());

    // //circle
    // println!("{}", circle.draw());
    // println!("Area: {}", circle.get_area());
}   