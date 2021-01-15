#![allow(dead_code)]

use std::io::{self, BufRead};
use text_io::read;

fn cripteaza(a: u8, b: u8, s: String) -> String {
    return s
        .chars()
        .map(|x| {
            if x != ' ' {
                ((a * (x as u8 - 'A' as u8) + b) % 26 + 'A' as u8) as char
            } else {
                x
            }
        })
        .collect();
}

fn caesar_afin() {
    let a: u8 = read!();
    let b: u8 = read!();
    let s: String = read!();

    let text_criptat: String = cripteaza(a, b, s);

    println!("Textul criptat este: {}", text_criptat)
}

fn caesar_afin_block() {
    let a: u8 = read!();
    let b: u8 = read!();
    let mut s: String = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    let lblock: usize = read!();

    let blocuri: Vec<String> = (0..s.len())
        .step_by(lblock)
        .into_iter()
        .map(|x| String::from(s.as_str().get(x..(x + lblock)).unwrap()))
        .collect();

    let text_criptat: String = blocuri
        .clone()
        .into_iter()
        .map(|x| cripteaza(a, b, x))
        .collect();
    println!(
        "Blocurile sunt: {:?} \nTextul criptat este: {}",
        blocuri, text_criptat
    )
}

fn main() {
    // caesar_afin();
    caesar_afin_block();
}
