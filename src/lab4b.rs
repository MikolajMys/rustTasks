//Zadanie 1 Napisz funkcję o nagłówku: fn co_drugi_znak(napis: ...) -> ...
// która zwróci napis zawierający co drugi znak z danego napisu:
pub fn co_drugi_znak(napis: String) -> String{
    napis.chars().step_by(2).collect()
}

//Zadanie 2 Zdefiniuj funkcję o nagłówku: fn szyfruj(napis: ..., klucz: ...) -> ...
// która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów.
// Przykład: szyfruj("Aladyn", 2) == "lAdany":


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


//Zadanie 2 Napisz funkcję, przyjmującą w argumencie napis i zwracającą ten sam napis, czytany od tyłu: