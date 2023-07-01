use std::io;

fn main(){
    println!("Masukan nama anda: "); 

    let mut nama = String::new();
    let mut umur: Option<u32> = None;
    let mut sekolah = String::new(); 
    let mut kota = String::new();

    io::stdin().read_line(&mut nama).expect("Terjadi kesalahan");
    
    println!("Masukan umur anda: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Terjadi kesalahan");

    println!("Masukan sekolah anda: ");

    io::stdin().read_line(&mut sekolah).expect("Terjadi kesalahan");

    println!("Masukan kota anda: ");
    
    io::stdin().read_line(&mut kota).expect("Terjadi kesalahan");

    let trimmed = input.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            umur = Some(i)
        }  
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    // Output Hasil
    print!("Nama kamu: {}", nama);
    if let Some(umur) = umur {
        print!("Umur kamu: {}\n", umur);
    } else {
        println!("Umur tidak tersedia.");
    }
    print!("Sekolah kamu: {}", sekolah);
    print!("Kota kamu: {}", kota);
} 