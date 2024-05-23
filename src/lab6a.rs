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
pub fn fraction(numerator: i32, denominator: i32) -> Option<f32>{
    if denominator == 0{
        None
    } else{
        Some(numerator as f32/denominator as f32)
    }
}

//Zadanie 2 Napisz funkcję position(element: i32, array: &[i32] -> Option<usize>).
// Funkcja powinna zwrócić indeks elementu w tablicy lub None, jeżeli element nie jest w tablicy:
pub fn position(element: i32, array: &Vec<i32>) -> Option<usize>{
    for i in 0..array.len(){
        if array[i] == element{
            return Some(i);
        }
    }
    None
}

//Zadanie 3 Napisz funkcję divisors(number: Option<u32>) -> usize,
// która zwróci liczbę dzielników parametru number lub zakończy działanie programu, jeśli number ma wartość None:
pub fn divisors(number: Option<u32>) -> usize{
    let mut dividers: Vec<u32> = Vec::new();
    let mut n = number.expect("zupa");
    for i in 1..n{
        if n % i == 0{
            dividers.push(i);
        }
    }
    dividers.len() as usize
}

//Zadanie 4 Napisz funkcję wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String,
// która stworzy wizytówkę (zadanie 5.3), w której w przypadku braku imienia zostanie użyte imię Jan,
// a w przypadku braku nazwiska -- Kowalski:
pub fn wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String{
    let mut s: String = String::new();
    let imie_low = String::from(imie.unwrap_or(String::from("jan")).to_lowercase());
    let nazwisko_low = nazwisko.unwrap_or(String::from("KOWALSKI")).to_lowercase();

    s.push_str(&imie_low[0..1].to_uppercase());
    s.push_str(". ");
    s.push_str(&nazwisko_low[0..1].to_uppercase());
    s.push_str(&nazwisko_low[1..]);
    s
}

//Zadanie 5 Napisz funkcję miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>),
// która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej:
pub fn miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>){
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0{
        (None, None)
    } else if delta == 0.0{
        (Some(-b/2.0*a), None)
    } else{
        (Some(-b-delta.sqrt()/2.0*a), Some(-b+delta.sqrt()/2.0*a))
    }
}

//Zadanie 6 Napisz funkcję oceny(punkty: &[u32], &mut[Result<u8, u32>]),
// która przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej niż 100 punktów,
// należy na tej pozycji umieścić wartość, informującą o błędzie i podać liczbę punktów ponad progiem:
pub fn oceny(punkty: &[u32], wyniki: &mut[Result<u8, u32>]){
    for(i, &punkt)in punkty.iter().enumerate(){
        if punkt > 100{
            wyniki[i] = Err(100+punkt);
        } else{
            let ocena = if punkt > 50{
                2
            } else if punkt > 60{
                3
            } else if punkt > 70{
                4
            } else if punkt > 80{
                5
            } else{
                6
            };
            wyniki[i] = Ok(ocena);
        }
    }
}

//Zadanie 7 Napisz funkcję rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>]),
// która przyjmie tablicę liczb zapisanych w postaci napisów w systemach dziesiętnym i szesnastkowym.
// Funkcja powinna rozpoznać system, w którym zapisana jest liczba i przekonwertować ją do zmiennej typu u32.
// Przyjmij, że liczby szesnastkowe oznaczone są prefiksem 0x. Nie wszystkie napisy muszą być poprawne; zadbaj o obsługę błędów:
pub fn rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>]){
    for (i, &data) in arr.iter().enumerate(){
        if data.starts_with("0x"){
            let cln_data = data.get(2..).unwrap();
            let mut num = 0;
            for c in cln_data.chars(){
                if let Some(digit) = c.to_digit(16){
                    num = num * 16 + digit;
                } else{
                    out[i] = Err(0);
                    return;
                }
            }
            out[i] = Ok(num);
        } else{
            match data.parse::<u32>(){
                Ok(num) => out[i] = Ok(num),
                Err(_) => out[i] = Err(0),
            }
        }
    }
}

//Zadanie 8 Napisz funkcję dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>,
// która pisemnie doda dwie, dowolnie duże liczby naturalne. Jeśli któryś z napisów nie jest liczbą,
// funkcja powinna zwrócić odpowiedni błąd:
pub fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>{
    for c in a.chars(){
        if !c.is_digit(10){
            return Err(String::from("not a number"));
        }
    }
    for c in b.chars(){
        if !c.is_digit(10){
            return Err(String::from("not a number"));
        }
    }

    let mut result = String::new();
    let mut carry = 0;
    let max_len = std::cmp::max(a.len(), b.len());
    let mut ar: Vec<char> = a.chars().rev().collect();
    let mut br: Vec<char> = b.chars().rev().collect();
    ar.resize(max_len, '0');
    br.resize(max_len, '0');
    for i in 0..max_len {
        let temp_a = ar[i].to_digit(10).unwrap_or(0);
        let temp_b = br[i].to_digit(10).unwrap_or(0);
        let sum = temp_a + temp_b + carry;
        //result.push_str(&(sum % 10).to_string());
        result.push(std::char::from_digit(sum % 10, 10).unwrap());
        carry = sum / 10;
    }
    if carry > 0 {
        result.push_str(&carry.to_string());
    }
    Ok(result.chars().rev().collect())
}