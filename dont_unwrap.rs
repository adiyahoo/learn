/*
Definisi:

unwrap() adalah metode dalam Rust yang digunakan untuk mengambil nilai sukses dari tipe Result<T, E>.
Ketika digunakan, unwrap() akan mengembalikan nilai sukses jika Result berisi nilai sukses, namun akan menyebabkan panic dan menghentikan program jika Result berisi nilai kesalahan.

Saran:

Tidak disarankan menggunakan unwrap() secara berlebihan dalam kode produksi.
Lebih baik menggunakan metode penanganan kesalahan yang lebih aman dan eksplisit, seperti match, if let, atau metode lain yang tersedia dalam Rust.
Metode-metode ini memungkinkan Anda untuk secara langsung menangani nilai kesalahan dan mengambil tindakan yang sesuai tanpa menyebabkan panic.

Penjelasan unwrap():

unwrap() adalah cara cepat dan sederhana untuk mengambil nilai sukses dari Result.
Namun, penggunaan berlebihan unwrap() dapat menyebabkan program panic dan menghentikan eksekusi jika terjadi kesalahan.
Penggunaan unwrap() sebaiknya dibatasi pada kasus-kasus di mana Anda yakin bahwa Result akan selalu berisi nilai sukses.

Rekomendasi dalam kode produksi:

Lebih baik menggunakan metode penanganan kesalahan yang lebih eksplisit, seperti match, untuk menangani Result.
Dengan menggunakan match, Anda dapat secara eksplisit menangani kasus sukses dan kesalahan, memberikan pesan kesalahan yang informatif, dan mengambil tindakan yang sesuai dalam setiap kasus.
Penggunaan match atau metode penanganan kesalahan lainnya memberikan kontrol yang lebih besar dan membantu mencegah panic yang tidak terduga.
Ini memungkinkan Anda untuk mengelola kesalahan dengan lebih baik dan memberikan pengalaman yang lebih andal dan aman dalam aplikasi produksi.
Jadi, untuk dokumentasi kode produksi atau kerja, disarankan untuk menghindari penggunaan berlebihan unwrap() dan lebih memilih metode penanganan kesalahan yang lebih aman dan eksplisit seperti match.
 */


fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Pembagi tidak boleh nol."));
    }

    Ok(a / b)
}

fn main() { 
    match bagi(10, 0) { 
             Ok(hasil) => println!("Hasil: {}", hasil),
             Err(hasil) =>  println!("Terjadi kesalahaan: {}", hasil)
    }   
}