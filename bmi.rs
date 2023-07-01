fn bmi(berat: f32, tinggi: f32) -> f32 {
    let tinggi_m: f32 = tinggi / 100.0;
    let bmi: f32 = berat / tinggi_m.powf(2.0);
    return bmi;
}

fn kategori_bmi(bmi: f32) -> &'static str {
    match bmi {
        bmi if bmi < 18.5 => "Berat badan kurang",
        bmi if bmi < 25.0 => "Berat badan normal",
        bmi if bmi < 30.0 => "Berat badan berlebih",
        _ => "Obesitas",
    }
}

fn main() {
    let berat: f32 = 65.0;
    let tinggi: f32 = 170.0;

    let bmi: f32 = bmi(berat, tinggi);
    let kategori: &str = kategori_bmi(bmi);

    println!("BMI Anda adalah: {}", bmi);
    println!("Kategori BMI: {}", kategori);
}
