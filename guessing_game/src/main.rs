use std::{cmp::Ordering, io};

use rand::Rng;

// entry point
fn main() {
    println!("Tebak angka!");

    loop {
        // generating random number
        let secret_number = rand::thread_rng().gen_range(1..=10);
        println!("Masukkan angka: ");
        // io::stdout().flush().unwrap(); // ketika print! tidak di-flush, tidak akan muncul di terminal langsung (buffered)

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

        // trim -> menghilangkan whitespace di awal dan akhir string
        // parse -> mengubah string menjadi target type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❗Harap masukkan angka :");
                continue;
            }
        };

        // lakukan perbandingan antara inputan dan angka acak
        // cmp() -> method untuk membandingkan 2 nilai
        // Ordering -> enum yang berisi 3 nilai: Less, Greater, Equal
        // match -> keyword untuk melakukan pattern matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Benar! Anda menang!");
                break;
            }
        }

        println!("Angka yang kamu masukkan: {}", guess);
        println!("Angka rahasia: {}\n", secret_number);
    }
}

// flush explanation
// https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
