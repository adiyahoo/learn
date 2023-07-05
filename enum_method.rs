enum Day { 
    Senin, Selasa, Rabu, Kamis, Jumat, Sabtu, Minggu
}

impl Day { 
    fn hari_kerja(&self) -> bool { 
        match self { 
            &Day::Sabtu | &Day::Minggu => return false, 
            _  => return true
        }
    }
}

fn main() { 
    let d = Day::Selasa;

    println!("Apakah ini hari kerja? {}", d.hari_kerja());
}