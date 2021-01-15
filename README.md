# cripto-lab-2020-2021

Exercitii pentru laboratorul de criptografie rezolvate in RUST

## Introducere

Rust este un limbaj de programare multi-paradigm, iar ca trasatura principala este importanta pentru siguranta ( evitarea coruperii memoriei, race condition). Sintaxa limbajului este similara cu `C\C++` si cu influente din limbaje precum `Haskell\OCaml\C#`. [Mai multe detalii](<https://ro.wikipedia.org/wiki/Rust_(limbaj_de_programare)>)

Limbajul a fost create Moziile Research cu scopul de a crea un motor de browser mai performant - [Servo](https://servo.org/). Sintaxa incarcata prezinta un obstacol atunci cand cineva incearca sa invete acest limbaj de programarea, dar aceasta aduce si un foarte mare beneficiu - programatorul poate sa-si exprime mai bine intentiile asupra compilatorului, fiecare decizie find explicita. De asemenea, compilatorul se asigura ca programatorul a folosit bunele practici pentru dezvoltare, el fiind capabil sa detecteze posibile erori inca din timpul compilarii.

Principalele arii de dezvoltare in care este folosit Rust sunt: unelte de tip CLI
, server, microprocesoare, aplicatii web.

**Exemple de aplicatii pot fi gasite pe aceasta pagina: https://github.com/rust-unofficial/awesome-rust**

## Resurse

- Site principal: https://www.rust-lang.org/
- Documentatia oficiala: https://doc.rust-lang.org/book/title-page.html
- Forum: https://internals.rust-lang.org/
- Reddit: https://www.reddit.com/r/rust/

## Exercitii

Pentru rularea exercitiilor trebuie sa instalati [compilatorul](https://www.rust-lang.org/tools/install). Intrati in fisierul exercitiului si rulati urmatoarea comanda:

```bash
cargo run
```

Pentru RSA, trebuie sa adaugati `--` pentru a face posibila transmiterea de parametri. Exemplu:

```bash
cargo run -- --help # afiseaza informatii despre comenzi
cargo run -- -m k -p 5 -q 3 cheie # genereaza fisierele pentru cheia publica, respectiv privata
cargo run -- cript -k cheie.puk mesaj.txt # afiseaza mesajul criptat folosind cheaia publica si fisierul de intrare
cargo run -- decript -k cheie.prk mesaj_criptat.txt # afiseaza mesajul decriptat folosind cheaia privata si fisierul de intrare
```
