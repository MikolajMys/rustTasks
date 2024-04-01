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
pub fn collatz_steps(i:u128, mut step:u128) -> u128{
    if i != 1{
        if i % 2 == 0{
            step += 1;
            collatz_steps(i/2, step)
        } else{
            step += 1;
            collatz_steps(3 * i + 1, step)
        }
    } else{
        step
    }
}
//Zadanie 4 Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga:
fn count_digits(n:u128) -> u128{
    let mut num:u128 = n;
    let mut sum:u128 = 0;
    while num > 0{
        sum += 1;
        num /= 10;
    }
    sum
}
pub fn is_armstrong_num(n:u128){
    let mut count:u128 = count_digits(n);
    let mut num:u128 = n;
    let mut sum:u128 = 0;
    while num > 0{
        let digit = num % 10;
        sum += digit.pow(count as u32);//** daje błąd
        num /= 10;
    }
    if sum == n{
        println!("Yes");
    } else{
        println!("No");
    }
}

//Zadanie 5 Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą:
fn find_dividers(n:u128) -> Vec<u128>{
    let mut dividers: Vec<u128> = Vec::new();
    for i in 1..n{
        if n % i == 0{
            dividers.push(i);
        }
    }
    dividers
}
pub fn is_perf_num(n:u128) {
    let dividers: Vec<u128> = find_dividers(n);
    let sum: u128 = dividers.iter().sum();
    if sum == n{
        println!("Yes")
    } else {
        println!("No");
    }
}
//Zadanie 6 Napisz funkcję, która wyświetli rozkład podanej liczby na czynniki pierwsze:
pub fn prime_factors(mut n: u64) {
    let mut factor = 2;
    while n > 1 {
        if n % factor == 0 {
            print!("{} ", factor);
            while n % factor == 0 {
                n /= factor;
            }
        }
        factor += 1;
    }
}
//Zadanie 7 Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128,
// która obliczy (x^n)%p w taki sposób, by działało to prawidłowo dla jak największych danych:
