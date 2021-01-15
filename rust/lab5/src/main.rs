#![allow(dead_code)]

use text_io::read;

fn putere_modul(a: usize, i: usize, p: usize) -> usize {
    let mut result = 1;
    for _ in 0..i {
        result = (result % p) * a;
    }

    return result % p;
}

fn gamal_tabel() {
    let p: usize = read!();
    let puteri: Vec<usize> = (1..p).into_iter().collect();

    let generatori: Vec<Vec<usize>> = (1..p)
        .into_iter()
        .map(|x| puteri.iter().map(|i| putere_modul(x, *i, p)).collect())
        .collect();

    println!("Table puterilor pentru Z{}* este:", p);
    for (pos, e) in generatori.iter().enumerate() {
        println!("{}: {:?}", pos + 1, e);
    }

    ()
}

fn log_discret() {
    let baza: usize = read!();
    let rez: usize = read!();
    let modul: usize = read!();

    let mut solutie: usize = 0;
    let mut are_solutie: bool = false;

    let mut x = baza;
    for i in 1..=modul {
        x = (x * baza) % modul;
        if x == rez {
            solutie = i + 1;
            are_solutie = true;
        }
    }

    if are_solutie {
        println!(
            "Solutie: log_{}({}) modulo {} = {}",
            baza, rez, modul, solutie
        );
    } else {
        println!("Problema nu are solutii")
    }
}

fn criptare_rsa() {
    let x: u32 = read!("Mesaj {}");
    println!("{}", x);
}

fn main() {
    // gamal_tabel();
    // log_discret();
    criptare_rsa();
}
