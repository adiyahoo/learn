use std::io;
use rand::Rng;

fn main() { 
    println!("Selamat datang di game flip coin 2 player");

    let mut player_one = String::new(); 
    let player_one_tipe: bool;

    let mut player_two = String::new();
    let player_two_tipe: bool;
    
    loop {  // player one
        println!("Player 1 input, pilih angka / gambar: ");
        match io::stdin().read_line(&mut player_one) {
            Ok(_) => {
                player_one = player_one.trim().to_lowercase();
                match player_one.as_str() { 
                    "gambar" => { 
                        player_one_tipe = true;
                        break;
                    }

                    "angka" => { 
                        player_one_tipe = false; 
                        break; 
                    }

                    _ => { 
                        println!("Tolong input angka / gambar")
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
            }
        }   
    }

    loop { 
        println!("Player 2 input, pilih angka / gambar: "); 
        match io::stdin().read_line(&mut player_two) { 
            Ok(_) => { 
                player_two = player_two.trim().to_lowercase(); 
                match player_two.as_str() { 
                    "gambar" => {
                        player_two_tipe = true;
                        break;
                    }

                    "angka" => {
                        player_two_tipe = false; 
                        break;
                    }

                    _ => {
                         println!("Tolong input angka / gambar")
                    }
                }
            }

            Err(e) => { 
                println!("Error reading input: {}", e)
            }
        }
    }

    let mut rng = rand::thread_rng();
    let random_bool = rng.gen_bool(0.5);

    match random_bool { 
        true => { 
            if player_one_tipe == player_two_tipe { 
                println!("Kelian beruda seri: {}", random_bool);
            } else if player_one_tipe { 
                println!("Player 1 menang memilih {}", player_one);
            } else if player_two_tipe { 
                println!("Player 2 menang memilih {}", player_two);
            }       
        }

        false => { 
            if player_one_tipe == player_two_tipe { 
                println!("Kelian beruda seri: {}", random_bool);
            } else if !player_one_tipe { 
                println!("Player 1 menang memilih {}", player_one);
            } 
            else if !player_two_tipe { 
                println!("Player 2 menang memilih {}", player_two);
            }
        } 
    }

}