#![allow(dead_code)]
use text_io::read;

fn fibo_sir() {
    let n: usize = read!();

    let mut sir = vec![1, 1];

    for i in 2..n {
        let nr_urm = sir[i - 1] + sir[i - 2];
        sir.push(nr_urm);
    }

    print!("Sir {:?}", sir);
}

fn prim() {
    let n: u32 = read!();

    fn este_prim(nr: u32) -> bool {
        (2..=((nr as f64).sqrt() as u32)).all(|x| nr % x != 0)
    }

    if este_prim(n) {
        println!("Numarul este prim.")
    } else {
        println!("Numarul nu este prim.")
    }
}

fn main() {
    prim()
}
