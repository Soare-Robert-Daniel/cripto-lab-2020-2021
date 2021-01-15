use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use text_io::scan;

struct Puk {
    e: u64,
    n: u64,
}

struct Prk {
    d: u64,
    n: u64,
}

struct Cheie {
    publica: Puk,
    privata: Prk,
}

fn genereaza_chei(p: u64, q: u64) -> Cheie {
    println!("[Cheie] p = {}, q = {}", p, q);

    let n = p * q;
    let phi = (p - 1) * (q - 1);

    fn cmmdc(n: &u64, x: &u64) -> u64 {
        let mut cn: u64 = *n;
        let mut cx: u64 = *x;
        while cn != cx {
            if cn > cx {
                cn = cn - cx;
            } else {
                cx = cx - cn;
            }
        }
        return cn;
    }
    let e: u64 = {
        let mut i: u64 = 3;

        while i < phi - 1 {
            if cmmdc(&phi, &i) == 1 {
                break;
            }
            i += 1;
        }

        i
    };

    println!("[Cheie] Exponentul de criptare este: {}", e);

    let d: u64 = {
        let mut i: u64 = 1;

        while i < phi {
            if (i * e) % phi == 1 {
                break;
            }
            i += 1;
        }

        i
    };

    println!("[Cheie] Exponentul de decriptare este: {}", d);

    Cheie {
        publica: Puk { e, n },
        privata: Prk { d, n },
    }
}

fn putere_modul(a: u64, e: u64, n: u64) -> u64 {
    let mut result = 1;
    for _ in 0..e {
        result = (result % n) * a;
    }

    return result % n;
}

fn cripteaza_mesaj(cheie: Puk, mesaj_numeric: Vec<u64>) -> Vec<u64> {
    mesaj_numeric
        .into_iter()
        .map(|m| putere_modul(m, cheie.e, cheie.n))
        .collect()
}

fn decripteaza_mesaj(cheie: Prk, mesaj_numeric: Vec<u64>) -> Vec<u64> {
    mesaj_numeric
        .into_iter()
        .map(|m| putere_modul(m, cheie.d, cheie.n))
        .collect()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let matches = App::new("rsa")
        .version("1.0")
        .author("Roberto")
        .about("Implementare RSA")
        .arg(
            Arg::new("mod")
                .short('m')
                .long("mod")
                .takes_value(true)
                .value_name("MOD")
                .about("Seteaza modul: 'k' pentru generarea cheii"),
        )
        .arg(
            Arg::new("p")
                .short('p')
                .takes_value(true)
                .value_name("P")
                .about("Primul numar prim"),
        )
        .arg(
            Arg::new("q")
                .short('q')
                .takes_value(true)
                .value_name("Q")
                .about("Al doilea numar prim"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .takes_value(true)
                .long("output")
                .about("Numele fisierului in care rezultatul va fi scris."),
        )
        .subcommand(
            App::new("cript")
                .about("Cripteaza mesaj")
                .arg(
                    Arg::new("key")
                        .about("Fisierul care contine cheia de criptare")
                        .short('k')
                        .long("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("KEY_FILE"),
                )
                .arg(
                    Arg::new("mesaj")
                        .about("Numele fisierului care contine mesajul")
                        .required(true)
                        .value_name("MESAJ_FILE"),
                ),
        )
        .subcommand(
            App::new("decript")
                .about("Decripteaza mesaj")
                .arg(
                    Arg::new("key")
                        .about("Fisierul care contine cheia de decriptare")
                        .short('k')
                        .long("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("KEY_FILE"),
                )
                .arg(
                    Arg::new("mesaj")
                        .about("Numele fisierului care contine mesajul criptat")
                        .required(true)
                        .value_name("MESAJ_FILE"),
                ),
        )
        .get_matches();

    if let Some(c) = matches.value_of("mod") {
        match c {
            "k" => {
                println!("Genereaza cheia private si cea publica.");
                // let cheie = genereaza_chei(p: u64, q: u64)

                let mut p: u64 = 2 as u64;
                let mut q: u64 = 3 as u64;

                match matches.value_of("p") {
                    Some(x) => match x.parse::<u64>() {
                        Ok(i) => p = i,
                        Err(_) => println!("Nu se poate procesa valuarea lui p"),
                    },
                    None => {
                        println!("Eroare de citie pentru p")
                    }
                }

                match matches.value_of("q") {
                    Some(x) => match x.parse::<u64>() {
                        Ok(i) => q = i,
                        Err(_) => println!("Nu se poate procesa valuarea lui q"),
                    },
                    None => {
                        println!("Eroare de citie pentru q")
                    }
                }

                let cheie = genereaza_chei(p, q);

                println!(
                    "Cheia publica este Puk = ({}, {})",
                    cheie.publica.e, cheie.publica.n
                );
                println!(
                    "Cheia privata este Prk = ({}, {})",
                    cheie.privata.d, cheie.privata.n
                );

                if let Some(o) = matches.value_of("output") {
                    let file_puk_name = &*(format!("{}.puk", o));
                    let file_prk_name = &*(format!("{}.prk", o));

                    let path_puk = Path::new(&file_puk_name);
                    let path_prk = Path::new(&file_prk_name);

                    let mut file_puk = match File::create(&path_puk) {
                        Ok(file) => file,
                        Err(why) => {
                            panic!("Nu s-a putut crea/deschide {}: {}", path_puk.display(), why)
                        }
                    };

                    let mut file_prk = match File::create(&path_prk) {
                        Ok(file) => file,
                        Err(why) => {
                            panic!("Nu s-a putut crea/deschide {}: {}", path_puk.display(), why)
                        }
                    };

                    match file_puk.write_all(
                        (format!("{} {}", cheie.publica.e, cheie.publica.n))
                            .as_str()
                            .as_bytes(),
                    ) {
                        Ok(_) => println!("Cheia publica a fost scris in {}", path_puk.display()),
                        Err(why) => panic!("Nu s-a putut scrie in {}: {}", path_puk.display(), why),
                    }

                    match file_prk.write_all(
                        (format!("{} {}", cheie.privata.d, cheie.privata.n))
                            .as_str()
                            .as_bytes(),
                    ) {
                        Ok(_) => println!("Cheia privata a fost scris in {}", path_puk.display()),
                        Err(why) => panic!("Nu s-a putut scrie in {}: {}", path_puk.display(), why),
                    }
                };
            }
            _ => println!("Mod invalid."),
        }
    }

    match matches.subcommand_name() {
        Some("cript") => {
            println!("Cripteaza mesaj.");

            let mut e: u64 = 0;
            let mut n: u64 = 0;
            let mut mesaj: Vec<String> = vec![];

            if let Some(ref matches) = matches.subcommand_matches("cript") {
                match matches.value_of("key") {
                    Some(key_file_name) => {
                        if let Ok(lines) = read_lines(key_file_name) {
                            for line in lines {
                                if let Ok(text) = line {
                                    scan!(text.bytes() => "{} {}", e ,n);
                                }
                            }
                        }
                    }
                    None => panic!("Numele fisierului pentru cheie este invalid"),
                }

                match matches.value_of("mesaj") {
                    Some(mesaj_file_name) => {
                        if let Ok(lines) = read_lines(mesaj_file_name) {
                            for line in lines {
                                if let Ok(text) = line {
                                    mesaj = [
                                        &mesaj[..],
                                        &(text
                                            .split(" ")
                                            .map(String::from)
                                            .collect::<Vec<String>>()),
                                    ]
                                    .concat();
                                }
                            }
                        }
                    }
                    None => panic!("Numele fisierului pentru cheie este invalid"),
                }
            }

            println!("Cheia publica este Puk = ({}, {})", e, n);
            println!("Mesajul este: {:?}", mesaj);

            let mesaj_criptat: Vec<String> = cripteaza_mesaj(
                Puk { e, n },
                mesaj
                    .into_iter()
                    .map(|m| m.parse::<u64>().unwrap())
                    .collect(),
            )
            .into_iter()
            .map(|m| m.to_string())
            .collect();

            println!("Mesajul criptat este: {:?}", mesaj_criptat);
        }
        Some("decript") => {
            println!("Decripteaza mesaj.");

            let mut d: u64 = 0;
            let mut n: u64 = 0;
            let mut mesaj_criptat: Vec<String> = vec![];

            if let Some(ref matches) = matches.subcommand_matches("decript") {
                match matches.value_of("key") {
                    Some(key_file_name) => {
                        if let Ok(lines) = read_lines(key_file_name) {
                            for line in lines {
                                if let Ok(text) = line {
                                    scan!(text.bytes() => "{} {}", d ,n);
                                }
                            }
                        }
                    }
                    None => panic!("Numele fisierului pentru cheie este invalid"),
                }

                match matches.value_of("mesaj") {
                    Some(mesaj_file_name) => {
                        if let Ok(lines) = read_lines(mesaj_file_name) {
                            for line in lines {
                                if let Ok(text) = line {
                                    mesaj_criptat = [
                                        &mesaj_criptat[..],
                                        &(text
                                            .split(" ")
                                            .map(String::from)
                                            .collect::<Vec<String>>()),
                                    ]
                                    .concat();
                                }
                            }
                        }
                    }
                    None => panic!("Numele fisierului pentru cheie este invalid"),
                }
            }

            println!("Cheia publica este Puk = ({}, {})", d, n);
            println!("Mesajul este: {:?}", mesaj_criptat);

            let mesaj_decriptat: Vec<String> = decripteaza_mesaj(
                Prk { d, n },
                mesaj_criptat
                    .into_iter()
                    .map(|m| m.parse::<u64>().unwrap())
                    .collect(),
            )
            .into_iter()
            .map(|m| m.to_string())
            .collect();

            println!("Mesajul decriptat este: {:?}", mesaj_decriptat);
        }
        _ => println!("Subcomanda invalida"),
    }
}
