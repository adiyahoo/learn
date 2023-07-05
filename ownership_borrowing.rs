
fn send_ownership(nama: String) -> String { 
    nama.to_string().replace("adi", "rhani")
}

fn main() { 
    let mut nama = String::from("adi"); 
    nama = send_ownership(nama);
    println!("{}", nama);
}