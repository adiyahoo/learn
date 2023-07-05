mod source; 

fn main() { 
    source::print_pesan(String::from("adiyaksa"));
    // tidak bisa diakses / error, karena private function
    // source::private_fun();
}