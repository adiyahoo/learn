use std::io;

#[allow(dead_code)]
fn penambahan(x : i32, y : i32){
    let hasil = x + y;
    print!("Hasil: {}", hasil);
}

#[allow(dead_code)]
fn pembagian(x : i32, y : i32){
    let hasil = x / y; 
    print!("Hasil: {}", hasil);
}

#[allow(dead_code)]
fn pengurangan(x : i32, y : i32){ 
    let hasil = x - y;
    print!("Hasil: {}", hasil);
}

#[allow(dead_code)]
fn perkalian(x: i32, y : i32){ 
    let hasil = x * y;
    print!("Hasil: {}", hasil);
}

fn main()
{
    println!("Kalkulator Rust: \na.penambahan\nb.pembagian\nc.pengurangan\nd.perkalian\nPilih opsi: ");

    let mut input = String::new();   

    io::stdin()
        .read_line(&mut input)
        .expect("terjadi kesalahan di input");  

    input = input.trim().to_lowercase();

    if input != "a" && input != "b" && input != "c" && input != "d" {
        print!("Pilihan tidak tersedia!");
    } else {
        let mut input_first = String::new();
        let mut input_second = String::new(); 

        println!("Masukan angka petama: ");
        io::stdin() 
            .read_line(&mut input_first)
            .expect("terjadi kesalahan");  

        println!("Masukan angka kedua: "); 
        io::stdin()
            .read_line(&mut input_second)   
            .expect("terjadi kesalahan");

        let input_first: i32 = input_first
            .trim()
            .parse()
            .expect("Terjadi kesalahan di convert input_first");

        let input_second: i32 = input_second
            .trim()
            .parse()
            .expect("Terjadi kesalahan di convert input_second");
        
        match input.as_str(){
            "a" => pembagian(input_first, input_second),
            "b" => pembagian(input_first, input_second),
            "c" => pengurangan(input_first, input_second),
            "d" => perkalian(input_first, input_second),
            _ => println!("Pilihan tidak tersedia!"),
        }

    }
}