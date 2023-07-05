fn main() { 
    let array_data: [String; 5] = [
    String::from("adiyaksa"), String::from("jamal"),
    String::from("nabila"),String::from("dea"),
    String::from("mark")]; 

    let nama_pekerja: String = String::from("adiyaksa");
    let referensi_str: &str =  &nama_pekerja;

   println!("Found: {}", match data_pekerja(referensi_str, &array_data) {
    Some(data) =>  data,
    None => "Data tidak ketemu",
    }) ;
}

fn data_pekerja(nama: &str, array_data: &[String; 5]) -> Option<&'static str> { 
    match nama.trim().to_lowercase().as_str() { 
       nama if nama == array_data[0] => Some("Student In Smk 1 Palangkaraya And Class Software Engineer"),
       nama if nama == array_data[1] => Some("UI/UX Design"),
       nama if nama == array_data[2] => Some("Web Developer"),
       nama if nama == array_data[3] => Some("Game Developer"),
       nama if nama == array_data[4] => Some("AI Engineer"),
        _ => None
    }
}