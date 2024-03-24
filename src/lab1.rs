//Zadanie 1 //Napisz program, który wyświetla informację o przestępności danego roku.
pub fn info() {
    let rok = 2021;
    if rok%4==0 && rok%100!=0 || rok%400==0 {
        println!("{} jest przestepny", rok)
    } else {
        println!("{} nie jest przestepny", rok)
    }
}
//Zadanie 2 // Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.
pub fn num_of_days() {
    let mies = 02;
    let rok = 2025;
    let przestepny: bool;
    let dni;
    if rok%4==0 && rok%100!=0 || rok%400==0 {
        //println!("{} jest przestepny", rok)
        przestepny = true;
    } else {
        //println!("{} nie jest przestepny", rok)
        przestepny = false;
    }
    if mies == 2{
        if przestepny{
            dni = 29;
        } else{
            dni = 28;
        }
    } else if mies == 4 || mies == 6 || mies == 9 || mies == 11 {
        dni = 30;
    } else{
        dni = 31;
    }
    println!("liczba dni to: {}", dni);
}
//Zadanie 3 // Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita; F=32+(9/5)C
pub fn from_c_to_f() {
    let tempc = 28.3;
    //F=32+(9/5)C
    let x = (tempc * 9.0/5.0)+32.0;
    println!("{}°C to {}°F", tempc, x);
}
//Zadanie 4 // Odwrotnie do Zadania 3.
pub fn reverse() {
    let tempf = 82.94;
    //F=32+(9/5)C
    let x = (tempf - 32.0)*(5.0/9.0);
    println!("{}°F to {}°C", tempf, x);
}
//zadanie 5 // Napisz program, który dla danych dwóch poprawnych pór jednej doby
// (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów
// (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).
pub fn hour_difference() {
    let hour1 = 12;
    let min1 = 10;
    let sec1 = 20;
    let sum1 = (hour1 * 3600) + (min1 * 60) + sec1;
    println!("{}",sum1);
    let hour2 = 8;
    let min2 = 5;
    let sec2 = 10;
    let sum2 = (hour2 * 3600) + (min2 * 60) + sec2;
    println!("{}",sum2);
    let mut res;
    if sum1 > sum2{
        res = sum1 - sum2;
    } else{
        res = sum2 - sum1;
    }
    println!("{}",res);
    let hours = res / 3600;
    let minutes = (res % 3600) / 60;
    let remaining_seconds = res % 60;
    println!("Roznica godzin to: {}h:{}m:{}s", hours, minutes, remaining_seconds);

}
//Zadanie 6 // Napisz program, który oblicza silnię dla danej liczby.
pub fn factorial(n:i32) -> i32{
    //let n = 4;
    let mut result = 1;
    for i in 1..=n{
        result *= i;
    }
    // println!("{}! = {}", n, result);
    result
}
//Zadanie 7 // Napisz program, który wyświetla cyfry danej liczby całkowitej (od końca).
pub fn print_numbers_from_num(mut n:i32) {
    // let mut n = 2137;
    while n > 0{
        let digit = n % 10;
        println!("{digit}");
        n /= 10;
    }
}
//Zadanie 8 // Napisz program, który oblicza sumę cyfry danej liczby całkowitej.
pub fn sum_of_numbers_from_num(n:u32) -> u32{
    let mut num = n;
    let mut sum = 0;
    while num > 0{
        let digit = num % 10;
        sum += digit;
        num /= 10;
    }
    sum
}
//Zadanie 9 // Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana. Zakładamy, że 0 < a < b < c.
pub fn find_pythagorean_triples(n: u32) {
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if a * a + b * b == c * c {
                    println!("({a}, {b}, {c})");
                }
            }
        }
    }
}
