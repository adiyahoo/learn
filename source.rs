/* Public function, jika ingin public 
function maka tambah kan pub di belakang function */
pub fn print_pesan(name: String) { 
    println!("Hi: {}", name);
}

/* Private function, hapus pub jika ingin private */
fn private_fun() { 
    println!("ini adalah private function");
}