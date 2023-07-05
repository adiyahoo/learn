fn main() { 
    let tinggi = 5; 


    for i in 1..=tinggi { 
        for _l in 1..=i { 
            print!("*")
        }
        println!();
    }   

    println!("\n");
    for i in 0..tinggi {
        for _j in 0..(tinggi - i - 1) {
            print!(" ");
        }
        
        for _k in 0..(2 * i + 1) {
            print!("*");
        }
        
        println!();
    }
}