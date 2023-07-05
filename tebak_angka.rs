use rand::Rng;
use std::io;
 

/* To get input from user */
fn user_input() -> i32 {
    loop {
        let mut var_user_input: String = String::new();

        println!("Masukan angka tebakan anda dari (0 - 10): ");

        match io::stdin().read_line(&mut var_user_input) {
            Ok(_) => match var_user_input.trim().parse::<i32>() {
                Ok(data_in) => {
                    if data_in > 10 {
                        println!("Tidak boleh melebihi dari 10");
                    } else {
                        return data_in;
                    }
                }
                Err(e) => println!("Harap masukan angka!: {}", e),
            },

            Err(e) => panic!("Terjadi kesalahan ketika input: {}", e),
        }
    }
}

/* get random num */
fn random_num() -> i32 {
    let mut rng = rand::thread_rng();
    let random: i32 = rng.gen_range(0..=10);
    random // return
}

fn main() {
    println!("Game tebak angka");

    // cuma manggil 1 kali
    loop {
        let data_in: i32 = user_input();
        let random: i32 = random_num();

        if data_in == random {
            println!("Selamat kamu menang! jawabannya: {}", random);
            break;
        } else if data_in > random { 
            println!("Terlalu besar");
        } else if data_in < random { 
            println!("Terlalu kecil");
        } else {
            println!("Oops kamu kalah! jawabannya: {}", random);
        }
    }
}
