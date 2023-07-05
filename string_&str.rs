/*String
String adalah tipe data yang dapat diubah (mutable) yang mewakili urutan karakter atau string yang dapat diperpanjang atau dimodifikasi. Dalam Rust, String digunakan ketika Anda memerlukan string yang dapat diubah, seperti ketika Anda perlu memanipulasi atau memperbarui string secara dinamis.

Anda dapat membuat String baru dengan menggunakan fungsi String::new() atau menggunakan string literal yang dikonversi menjadi String dengan menggunakan metode String::from().

Contoh penggunaan String:

rust
Copy code
let mut my_string = String::new();
my_string.push_str("Hello, "); // Menambahkan string ke dalam `my_string`
my_string.push_str("Rust!"); // Menambahkan string lagi
println!("{}", my_string); // Output: Hello, Rust!
Dalam contoh ini, kita membuat my_string sebagai String kosong dengan menggunakan String::new(). Kemudian, kita menggunakan metode push_str() untuk menambahkan string ke dalam my_string. Akhirnya, kita mencetak my_string yang berisi "Hello, Rust!".

Anda dapat mengubah, memperbarui, atau memodifikasi isi dari String karena itu adalah tipe data yang mutable. Anda dapat menggunakan berbagai metode dan operasi yang disediakan oleh Rust untuk memanipulasi String.

str
str (dibaca sebagai "string slice") adalah tipe data referensi yang tidak dapat diubah (immutable) yang mengacu pada urutan karakter atau string yang sudah ada. str biasanya digunakan ketika Anda hanya perlu membaca atau menggunakan string, tanpa perlu melakukan perubahan pada string itu sendiri.

Anda dapat membuat str dengan menggunakan string literal, dengan mengambil pinjaman dari String menggunakan tipe &str, atau dengan memotong (slicing) string lain yang memiliki tipe str.

Contoh penggunaan str:

rust
Copy code
fn print_message(message: &str) {
    println!("{}", message);
}

fn main() {
    let string_literal: &str = "Hello, Rust!"; // Membuat `str` dari string literal
    print_message(string_literal); // Menggunakan `str` dalam fungsi
}
Dalam contoh ini, kita membuat string_literal sebagai &str yang mengacu pada string literal "Hello, Rust!". Kemudian, kita memanggil fungsi print_message dengan string_literal sebagai argumen.

Anda tidak dapat memodifikasi nilai str karena itu adalah tipe data yang immutable. Namun, Anda masih dapat melakukan operasi membaca (reading operations) pada str, seperti mengakses karakter individual atau menggunakan fungsi yang menerima str sebagai parameter.

Jadi, perbedaan utama antara String dan str adalah bahwa String adalah tipe data mutable yang dapat diubah, sementara str adalah tipe data referensi yang immutable yang mengacu pada urutan karakter yang sudah ada.

Semoga penjelasan ini berguna untuk dokumentasi Anda. Jika Anda memiliki pertanyaan lain atau memerlukan penjelasan tambahan, silakan beri tahu saya.*/

fn print_message(message: &str) {
    println!("{}", message);
}

fn main() {
    let mut my_string = String::new();
    my_string.push_str("Hello, "); // Menambahkan string ke dalam `my_string`
    my_string.push_str("Rust!"); // Menambahkan string lagi
    println!("{}", my_string); // Output: Hello, Rust!

    // &str
    let string_literal: &str = "Hello, Rust!"; // Membuat `str` dari string literal
    print_message(string_literal); // Menggunakan `str` dalam fungsi
}
