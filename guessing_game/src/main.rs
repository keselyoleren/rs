use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number luas balok dengan rumus panjang * lebar * tinggi!");

    let mut rng = rand::thread_rng();
    let panjang = rng.gen_range(1..=10);
    let lebar = rng.gen_range(1..=10);
    let tinggi = rng.gen_range(1..=10);
    let volume = panjang * lebar * tinggi;

    println!("Panjang: {}, Lebar: {}, Tinggi: {}", panjang, lebar, tinggi);

    loop {
        println!("Masukkan tebakan volume:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("gagal membaca input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Tolong masukkan angka yang valid!");
                continue;
            }
        };

        println!("Tebakan Anda: {}", guess);

        match guess.cmp(&volume) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Selamat! Tebakan Anda benar!");
                break;
            }
        }
    }    

}