#![allow(dead_code)]
use text_io::read;

fn numar_pasii(nr: u64) -> u32 {
    let mut x = nr;
    let mut count: u32 = 0;

    while x > 1 {
        x = if x % 2 == 0 { x / 2 } else { x * 3 + 1 };
        count += 1;
    }

    return count;
}

fn collatz() {
    let n: u64 = read!();

    println!("Numar de pasi: {}", numar_pasii(n))
}

fn collatz_big() {
    let mut nr_max_pasi = 0;
    let mut nr = 0;

    for i in 2..=10000000 {
        let nr_pasi = numar_pasii(i);

        if nr_pasi > nr_max_pasi {
            nr_max_pasi = nr_pasi;
            nr = i;
        }
    }

    println!(
        "Cel mai mare numar de pasi este {} pentru numarul {}",
        nr_max_pasi, nr
    );
}

fn main() {
    collatz_big();
}
