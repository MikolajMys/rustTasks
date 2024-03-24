//Zadanie 1 z zestawu 2a zmodyfikuj funkcje tak,
// by obliczenia były prowadzone nie dla sztywno zakodowanych funkcji i ich pochodnych,
// lecz by funkcję i pochodną można było przekazać do funkcji:
fn f(x: f64) -> f64{x*x-2.0}
fn fp(x: f64) -> f64{2.0*x}
pub fn met_newt_loop(f1:fn(f64) -> f64, f2:fn(f64) -> f64, x0:f64, eps:f64, n:u128) -> f64{
    let mut x = x0;
    let mut i = 0;
    loop{
        x = x-(f1(x)/f2(x));
        i += 1;
        if f1(x).abs() <= eps || i > n{
            return x;
        }
    }
}
//Zadanie 2 Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126):
pub fn print_ascii() {
    for code in 33..=126 {
        let character = code as u8 as char;
        println!("{}\t{}", character, code);
    }
}
//Zadanie 3 Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji,
// w której osiągamy jedynkę w problemie Collatza (np. dla n=12 wynikiem jest 9):
// pub fn collatz_steps(i:u128, step:u128) -> u128{
//
// }
//Zadanie 4 Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga:

//Zadanie 5 Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą:

//Zadanie 6 Napisz funkcję, która wyświetli rozkład podanej liczby na czynniki pierwsze:

//Zadanie 7 Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128,
// która obliczy (xn)%p w taki sposób, by działało to prawidłowo dla jak największych danych:
