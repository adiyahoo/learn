
struct Pekerja{
    id: u8,
    nama: String, 
    umur: u8, 
    total_absen: u128
}

impl Pekerja{
    fn profile(&self){
        println!(
            "id : {}, nama: {}, umur: {}, total-absen: {}", 
            self.id, self.nama, self.umur, self.total_absen
        );
    }

    fn hitung_gaji(&self) -> u128{
        let gaji_hari: u128 = 50000;
        let hasil: u128 = gaji_hari * self.total_absen;
        hasil // return
    }
}

fn main(){
    let pekerja1 = Pekerja {
        id: 1,
        nama: String::from("Adiyaksa"),
        umur: 15,
        total_absen: 30,
    }; 
    
    let pekerja2 = Pekerja {
        id: 2,
        nama: String::from("rhani"),
        umur: 15,
        total_absen: 128
    }; 

   pekerja1.profile();
   pekerja2.profile();
   println!("Gaji pekerja 1: {}", pekerja1.hitung_gaji());
   println!("Gaji pekerja 2: {}", pekerja2.hitung_gaji());
}