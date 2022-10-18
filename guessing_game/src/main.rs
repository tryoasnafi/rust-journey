use std::io::{self, Write};

// entry point
fn main() {
    println!("Tebak angka!");
    print!("Masukkan angka: ");
    io::stdout().flush().unwrap(); // ketika tidak di-flush, tidak akan muncul di terminal

    /*
        let -> keyword untuk deklarasi variabel
        mut -> keyword untuk membuat variabel mutable (dapat di-reassign)
        guess -> nama variabel
    */
    let mut guess = String::new();

    // & -> operator untuk mengambil referensi dari variabel
    io::stdin()
        .read_line(&mut guess)
        .expect("Gagal membaca inputan!");

    println!("Angka yang kamu masukkan: {}", guess);
}

// flush explanation
// https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
