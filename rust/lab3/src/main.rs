#![allow(dead_code)]

use text_io::read;

fn este_prim(nr: u32) -> bool {
    (2..=((nr as f64).sqrt() as u32)).all(|x| nr % x != 0)
}

fn ciurul_eratostene() {
    let n: usize = read!();
    let mut sunt_prime = vec![true; n + 1];

    if n > 1 {
        sunt_prime[1] = false;
    }

    for i in 2..(n + 1) {
        if este_prim(i as u32) {
            let mut multiplu = i * i;
            while multiplu <= n {
                sunt_prime[multiplu] = false;
                multiplu += i;
            }
        }
    }

    let ciur: Vec<usize> = sunt_prime
        .iter()
        .enumerate()
        .filter_map(|(index, &este_nr_prim)| if este_nr_prim { Some(index) } else { None })
        .collect();

    println!("Ciurul lui Eratostene este {:?}", ciur)
}

fn cmmdc(n: &usize, x: &usize)-> usize {
    let mut cn: usize = *n;
    let mut cx: usize = *x;
    
    while cn != cx {
        if cn > cx {
            cn = cn - cx;
        } else {
            cx = cx - cn;
        }
    }

    return cn;
}

fn putere_fermat(a: usize, n: usize) -> usize {
    let mut result = 1;
    for _ in 1..=n-1 {
        result = (result % n) * a;
    }

    return result;
}

fn testul_fermat() {
    let n: usize = read!();

    let verifica = (2..n).into_iter().all(|a| putere_fermat(a, n) % n == 1);

    if verifica {
        println!("{} este prim, conform testului Fermat.", n);
    } else {
        println!("{} nu este prim, conform testului Fermat.", n);
    }
}

fn main() {
    // ciurul_eratostene();
    testul_fermat();
}
