use std::io; 

fn main() { 
    let mut input = String::new(); 

    println!("hi mate going fuck: ");
            
    match io::stdin().read_line(&mut input) { 
        Ok(_) => {
           println!("what you say fuck: {}", input);
        }

        Err(e) => {
            println!("nigga {}", e);
        }
    }

    let mut input_int = String::new();

    println!("hi mate going fuck with number: ");

    match io::stdin().read_line(&mut input_int) {
        Ok(_) => {
            // Menghapus karakter newline dari string input
            input_int = input_int.trim().to_string();

            // Mengubah string menjadi tipe data integer
            match input_int.parse::<i32>() {
                Ok(parsed_int) => {
                    // String berhasil diubah menjadi integer
                    println!("Angka yang dimasukkan: {}", parsed_int);
                }
                Err(_) => {
                    // String tidak dapat diubah menjadi integer
                    println!("Input tidak valid. Masukkan angka!");
                }
            }
        }
        Err(error) => {
            // Error saat membaca input
            println!("Error: {}", error);
        }
    }

}