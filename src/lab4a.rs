//Zadanie 1 Napisz funkcję o nagłówku fn liczba_wystapien(napis: ..., znak: ...) -> ...,
// która zliczy i zwróci ile jest danych znaków w danym napisie:
pub fn liczba_wystapien(napis: &str, znak: char) -> i32{
    let mut sum:i32 = 0;
    for i in napis.chars(){
        if i == znak{
            sum += 1;
        }
    }
    sum
}
//Zadanie 2 Napisz funkcję o nagłówku fn rzymskie(napis: ...) -> ..., która dla napisu reprezentującego liczbę
// w zapisie rzymskim zwraca liczbę reprezentowaną przez ów napis: Przykład: // rzymskie("III") == 3
