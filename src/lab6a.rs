//Zadanie 1 Napisz funkcję o nagłówku
// fn zamien_syst8_na_syst2(z: &str) -> Option<String>
// zamieniającą zapis liczby całkowitej w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty.
// Wynik None ma oznaczać wystąpienie w parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze:


//Zadanie 2 Napisz funkcję o nagłówku: fn wartosc_syst2(z: &str) -> Option<u8>
// obliczającą wartość zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach.
// Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest None:


//Zadanie 3 Napisz funkcję o nagłówku: fn wartosc_syst8(z: &str) -> Option<u8>
// obliczającą wartość zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach.
// Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest None.
// Uwaga! Funkcję tę należy zbudować z funkcji z zadań poprzednich:

//Dodatkowe:
//Zadanie 1 Napisz funkcję fraction(numerator: i32, denominator: i32) -> Option<f32>,
// która wykona odpowiednie dzielenie lub zwróci None, jeżeli to niemożliwe:


//Zadanie 2 Napisz funkcję position(element: i32, array: &[i32] -> Option<usize>).
// Funkcja powinna zwrócić indeks elementu w tablicy lub None, jeżeli element nie jest w tablicy:


//Zadanie 3 Napisz funkcję divisors(number: Option<u32>) -> usize,
// która zwróci liczbę dzielników parametru number lub zakończy działanie programu, jeśli number ma wartość None:


//Zadanie 4 Napisz funkcję wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String,
// która stworzy wizytówkę (zadanie 5.3), w której w przypadku braku imienia zostanie użyte imię Jan,
// a w przypadku braku nazwiska -- Kowalski:


//Zadanie 5 Napisz funkcję miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>),
// która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej:


//Zadanie 6 Napisz funkcję oceny(punkty: &[u32], &mut[Result<u8, u32>]),
// która przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej niż 100 punktów,
// należy na tej pozycji umieścić wartość, informującą o błędzie i podać liczbę punktów ponad progiem:


//Zadanie 7 Napisz funkcję rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>]),
// która przyjmie tablicę liczb zapisanych w postaci napisów w systemach dziesiętnym i szesnastkowym.
// Funkcja powinna rozpoznać system, w którym zapisana jest liczba i przekonwertować ją do zmiennej typu u32.
// Przyjmij, że liczby szesnastkowe oznaczone są prefiksem 0x. Nie wszystkie napisy muszą być poprawne; zadbaj o obsługę błędów:
pub fn rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>]){
    for (i, &s) in arr.iter().enumerate(){
        if s.starts_with("0x"){
            let new = &s[2..];
            let x:Result<u32,u32> = Ok(u32::from_str_radix(new, 16).unwrap());
            out[i] = x;
        } else {
            let x = s.parse::<u32>();
            match x {
                Ok(val) => out[i] = Ok(val),
                Err(_) => out[i] = Err(0),
            }
        }
    }
}

//Zadanie 8 Napisz funkcję dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>,
// która pisemnie doda dwie, dowolnie duże liczby naturalne. Jeśli któryś z napisów nie jest liczbą,
// funkcja powinna zwrócić odpowiedni błąd: