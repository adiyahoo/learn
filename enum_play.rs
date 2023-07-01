use std::process::exit;

const MAX_LEVEL: u8 = 10;
const USER_BAN: [&str; 5] = ["adi", "rhani", "jamal", "agus", "amat"];

#[allow(dead_code)]
enum PlayerPos{
    Rumah,
    Lapangan,
    Taman,
    Bandara
}

#[allow(dead_code)]
struct Player{ 
    username: String,
    posisi: PlayerPos, 
    uang: i32, 
    level: u8,
    ban: bool
}

impl Player {
    fn cek_posisi(&self) -> &str {
        match self.posisi {
            PlayerPos::Rumah => "Rumah",
            PlayerPos::Lapangan => "Lapangan",
            PlayerPos::Taman => "Taman",
            PlayerPos::Bandara => "Bandara",
        }
    }

    fn cek_progress(&self) -> &str {
        match self.level {
           MAX_LEVEL => "Tamat",
           _ => "Belum tamat"
        }
    }

    fn cek_ban(&mut self) -> bool { 
        for b in 0..USER_BAN.len() {
            if self.username == USER_BAN[b]{
                self.ban = true;
                return true;
            }
        }
        return false;
    }
}

fn main(){
    let mut player = Player {
        username: String::from("adiyaksa"),
        posisi: PlayerPos::Bandara,
        uang: 10000,
        level: 8,
        ban: false
    };

    match player.cek_ban() { 
        true => {
            println!("Kamu diban");
            exit(1);
        }
        false => println!("Kamu tidak diban")
    };

    println!("Posisi player: {}", player.cek_posisi());
    println!("Progres player: {}", player.cek_progress());
}