// Reading a File
// use std::fs::File;
// use std::io::prelude::*;

// Command Line Arguments
// use std::env;

// Belajar enum
// #[allow(dead_code)]
// enum NamaPlayer{
//     Adi, 
//     Asep,
//     Jamal
// }

// Struct Simpel 
// struct Color{ 
//     red: u8, 
//     green: u8, 
//     blue: u8
// }

// Struct
// #[allow(non_snake_case)]
// struct DataSiswa { 
//     NamaSiswa: String, 
//     Umur: u8, 
//     Sekolah: String,
//     Alamat: String,
//     Kelas: String,
//     NilaiRata: f32
// }

// Tuple Structs
// struct AngkaKuSuka(u8, u8, u8);

// Constants
// const MAX_VALUE: u8 = 20;

// Impl keyword
// struct Rectangle{
//     width: u32,
//     height: u32
// }

// impl Rectangle{
//     fn print_deskripsi(&self){
//         println!("Rectangle {} x {}", self.width, self.height);
//     }

//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn is_square(&self) -> bool{
//         self.width == self.height
//     }
// }

// Implementing Traits
// struct Person { 
//     name: String, 
//     age: u32
// }

// impl ToString for Person { 
//     fn to_string(&self) -> String { 
//         format!("Name: {}, Age: {}", self.name, self.age)
//     }
// }

fn main(){
    // Contoh pembuatan variabel agar tidak immutable variabel 
    // let mut a: i64 = 0; 
    // println!("{}", a: i64);
    // a: i64 = 1;
    // println!("{}", a: i64);

    
    // let a: i64 = 13; // i64
    // // let f: f64 = 5.4; // f64
    // // let t: bool = false;

    // if a == 1
    // { 
    //     println!("hallo"); 
    // } 
    // else if a < 10
    // { 
    //     println!("sapi");
    // } else {
    //     println!("ayam");


    // Loop
    // let mut a: i32 = 0;

    // loop{ 
    //     a += 1;

    //     if a == 3{
    //         continue;
    //     }

    //     if a > 10{
    //         break;
    //     }

    //     println!("{}", a);
    // }

    // println!("Selesai");


    // While loop
    // let mut a: i32 = 1;

    // while a <= 50 {
    //     println!("{}", a);
    //     a+=1;
    // }

    // For loop
    // let _a = 1..11;
    // let nama = vec!["ADI", "JAMAL", "SAPUDIN"];

    // for (asu, nama) in nama.iter().enumerate(){
    //     println!("{} nama: {}", asu, nama);
    // }

    // let check_ban: NamaPlayer = NamaPlayer::Adi;

    // match check_ban {    
    //     NamaPlayer::Adi => println!("tidak diban"),
    //     NamaPlayer::Asep => println!("kamu diban"),
    //     NamaPlayer::Jamal => println!("tidak diban")
    // }
    
    // Constants
    // for i in 1..MAX_VALUE{
    //     println!("{}", i);
    // }

    // Tuples
    // let nilai = (100, "Aku mariang", 4.5, 400, 500); // 0 = 100, 1 = 200, 2 = 300, 3 = 400, 4 = 500

    // println!("{}", nilai.1);

    // Safe tuples
    // let nilai: (i32, i32, String, f32, bool) = (40, 50, String::from("Kocak"), 4.5, false);

    // println!("{}", nilai.2);

    // Function
    // print_number_to(10);

    // References
    // let mut x = 10; 
    // let nilai_saya = &mut x;
    // *nilai_saya = 100;
    // println!("{}", nilai_saya);

    // Struct
    // let mut output_datasiswa = DataSiswa { NamaSiswa: String::from("Adiyaksa"), Umur: 15, Sekolah: String::from("SMKN 1 PALANGKARAYA"), Alamat: String::from("Jalan Samudin Aman 4b No 03"), Kelas: String::from("X-PPLG1"), NilaiRata: 90.0};
    // output_datasiswa.NilaiRata = 95.0;
    // println!("Nama Siswa: {} \nUmur: {} \nSekolah: {} \nAlamat: {}\nKelas: {}\nNilai Rata-Rata: {}", output_datasiswa.NamaSiswa, output_datasiswa.Umur, output_datasiswa.Sekolah, output_datasiswa.Alamat, output_datasiswa.Kelas, output_datasiswa.NilaiRata);

    // Tuple Structs
    // let mut suka_angka = AngkaKuSuka(255, 100, 0);
    // suka_angka.0 = 150;
    // println!("{}", suka_angka.0);

    // Struct Simpel Dan pake function
    // let bg = Color{red:0, green: 0, blue: 255};

    // print_color(&bg);

    // Array
    // let number: [i32; 5] = [10, 20, 30, 40, 50];

    // for n in 0..number.len(){
    //     println!("Number: {}", number[n]);
    // }

    // Impl keyword
    // let my_rect = Rectangle {width: 10, height:5};
    
    // my_rect.print_deskripsi();
    // println!("Area: {}", my_rect.area());
    // println!("Rectangle is a square: {}", my_rect.is_square());

    // Implementing Traits
    // let my_traits = Person {name: String::from("John Doe"), age: 12};
    // println!("{}", my_traits.to_string()); 

    // Vectors
    // let mut data_vec = vec![1, 2, 3]; 

    // data_vec.remove(1); // remove '2'
    // data_vec.push(30);

    // for n in data_vec.iter() { 
    //     println!("{}", n);
    // }

    // // Vectors String 
    // let mut data_str: Vec<(i32, String)> = vec![
    //     (1, String::from("Rhani")),
    //     (2, String::from("Adi")),
    //     (3, String::from("Love")),
    // ];

    // data_str.push((4, String::from("Agus")));

    // for (number, text) in data_str.iter() {
    //     println!("Number: {}, Text: {}", number, text);
    // }

    // Reading a File
    // let mut file = File::open("test.txt").expect("tidak bisa buka file"); 
    // let mut c = String::new();

    // file.read_to_string(&mut c).expect("tidak bisa baca file");
    // println!("File content: {}", c);

    // Command Line Arguments
    // let token: Vec<String> = env::args().collect(); 

    // println!("{}", token[1]);

    
}

// Struct Simpel 
// fn print_color(c: &Color){
//     println!("Color - R {}, G {}, B {}", c.red, c.green, c.blue);
// }

// Function
// fn print_number_to(num: u32){
//     for n in 1..num{
//         if is_even(n){
//             println!("{} it is even!", n);
//         } else {
//             println!("{} is odd", n);
//         }
//     }
// }

// fn is_even(num: u32) -> bool{
//     return num % 2 == 0;
// }
