use rand::Rng; 

fn main() { 
    println!("Koin {}",  match rand::thread_rng().gen_bool(0.5) { 
        true => "Angka".to_string(),
        false => "Gambar".to_string(),
    });
}