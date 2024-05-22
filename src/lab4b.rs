//Zadanie 1 Napisz funkcję o nagłówku: fn co_drugi_znak(napis: ...) -> ...
// która zwróci napis zawierający co drugi znak z danego napisu:
pub fn co_drugi_znak(napis: String) -> String{
    napis.chars().step_by(2).collect()
}

//Zadanie 2 Zdefiniuj funkcję o nagłówku: fn szyfruj(napis: ..., klucz: ...) -> ...
// która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów.
// Przykład: szyfruj("Aladyn", 2) == "lAdany":
// fn szyfruj(napis: &str, klucz: i32) -> String{
//     let mut s: String = String::new();
//
// }

//Zadanie 3 Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko,
// a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku
// pierwsza litera imienia i nazwiska mają być duże, pozostałe małe.
// Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski":


//Zadanie 4 Napisz funkcję o nagłówku: fn na_rzymskie(liczba: ...) -> ... która dla danej liczby całkowitej zwraca jej zapis rzymski.
// Przykład: na_rzymskie(3) == "III":


//Zadanie 5 Napisz funkcję o nagłówku: fn dodaj_pisemnie(a: ..., b: ...) -> ...
// która doda dwie liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis.
// Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykład: dodaj_pismnie("1", "3") == "4":

//Dodatkowe:
//Zadanie 1 Napisz funkcję, zliczającą liczbę wystąpień pewnego znaku w napisie:
pub fn count_letters(letter: &str,text: &str) -> i32{
//     let mut x:i32 = 0;
//     for c in text.chars(){
//         if c == letter{
//             x += 1;
//         }
//     }
//     x
    return text.matches(letter).count() as i32;
}

//Zadanie 2 Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko,
// a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska,
// przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe.
// Na przykład, dla danych jan KOWALSKI = J. Kowalski:
pub fn wizytowka(imie: &str, nazwisko: &str) -> String{
    let mut s: String = String::new();
    let imie_low = String::from(imie.to_lowercase());
    let nazwisko_low = nazwisko.to_lowercase();
//     s.push_str(imie[0..1].to_uppercase().to_string().as_str());
//     s.push_str(". ");
//     s.push_str(nazwisko[0..1].to_uppercase().to_string().as_str());
//     s.push_str(nazwisko[1..].to_string().as_str());
//     s
    s.push_str(&imie_low[0..1].to_uppercase());
    s.push_str(". ");
    s.push_str(&nazwisko_low[0..1].to_uppercase());
    s.push_str(&nazwisko_low[1..]);
    s
//     s.push(imie_low.to_uppercase().chars().next().unwrap());
//     s.push_str(". ");
//     s.push(nazwisko_low.to_uppercase().chars().next().unwrap());
//     s.push_str(&nazwisko_low[1..]);
//     s
}
//Zadanie 3 Napisz funkcję, która na podstawie napisu tworzy napis, zawierający co drugi znak napisu podanego w argumencie:
pub fn skip_two(text: &str) -> String{
    //text.chars().step_by(2).collect()
    let mut z: String = String::new();
    for c in text.chars().step_by(2){
        z.push(c);
    }
    z
}
//Zadanie 4 Napisz funkcję, przyjmującą w argumencie napis i zwracającą ten sam napis, czytany od tyłu:
pub fn reverse(text: &str) -> String{
    //text.chars().rev().collect()
    let mut y = String::new();
    for i in 0..text.len() {
        y.push(text.chars().nth(text.len() - i - 1).unwrap_or('0'));
    }
    y
}
//Zadanie 5 Zdefiniuj funkcję o nagłówku fn szyfruj(napis: ..., klucz: ...) -> ...
// która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym —
// klucz określa długość odwracanych fragmentów. Przykład: szyfruj("Aladyn", 2) == "lAdany":
pub fn szyfruj(napis: &str, klucz: usize) -> String{
    let mut code: String = String::new();
    let mut offset = 0;
    loop{
        if offset * klucz > napis.len(){
            break;
        }
        code.push_str(napis.chars()
            .skip(offset * klucz)
            .take(klucz)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            .as_str());
        offset += 1;
    }
    code
}
//Zadanie 6 Napisz funkcję o nagłówku fn dodaj_pisemnie(a: ..., b: ...) -> ...
// która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym —
// i zwróci wynik również jako napis. Przykład:("1", "3") == "4":
pub fn dodaj_pisemnie(a: &str, b: &str) -> String {
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
        result.push(std::char::from_digit((sum % 10), 10).unwrap());
        carry = sum / 10;
    }
    if carry > 0 {
        result.push_str(&carry.to_string());
    }
    result.chars().rev().collect()
}