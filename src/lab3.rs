//Zadanie 1 Napisz funkcję dwuargumentową, która zamieni wartości podanych w argumentach zmiennych
// (dla ustalenia uwagi: typu i32):
pub fn swap_two_args(x: &mut i32, y: &mut i32){
    let temp: i32 = *x;
    *x = *y;
    *y = temp;
}
//Zadanie 2 Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów
// (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco:
pub fn swap_sort_three_args(x: i32, y: i32, z: i32){
    let mut arr: [i32; 3] = [x,y,z];
    println!("{:?}", arr);
    if arr[0] < arr[1]{
        let temp: i32 = arr[0];
        arr[0] = arr[1];
        arr[1] = temp;
    }
    if arr[1] < arr[2]{
        let temp: i32 = arr[1];
        arr[1] = arr[2];
        arr[2] = temp;
    }
    if arr[0] < arr[1]{
        let temp: i32 = arr[0];
        arr[0] = arr[1];
        arr[1] = temp;
    }
    println!("{:?}", arr);
}
//Zadanie 3 Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.
// fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...:
pub fn rand(seed: &mut i128, min_rand: i128, max_rand: i128) -> i128{
    *seed = (75 * *seed + 70) % 6553;
    //println!("{}", *seed);
    *seed % (max_rand-min_rand+1) + min_rand
}
//Zadanie 4 Napisz funkcję swap_arr(arr: ..., i: usize, j: usize),
// która zamieni wartości dwóch podanych elementów pewnej tablicy:
pub fn swap_arr(arr: &mut [i32], i: usize, j: usize){
    let temp: i32 = arr[i-1];
    arr[i-1] = arr[j-1];
    arr[j-1] = temp;
}

//Zadanie 5 Stwórz funkcję rand_perm(arr: ..., seed: ...),
// permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie:
pub fn rand_perm(arr: &mut [i32], seed: &mut i128){
    for i in 0..arr.len() {
        let j = rand(seed, 0, (arr.len() - 1) as i128) as usize;
        swap_arr(arr, i, j);
        println!("{j}");
    }
}

//Inna wersja zadań:
//Zad 1 Napisz funkcję d2((x, y), (x, y)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^2.

//Zad 2 Napisz funkcję d3((x, y, z), (x, y, z)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^3.

//Zad 3 Stwórz tablicę N elementów, którą wypełnisz resztami z dzielenia liczby 100 przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości tablicy od końca.

//Zad 4 Napisz funkcję avg(&[u32]) -> f32, która obliczy średnią arytmetyczną liczb z tablicy.

//Zad 5 Napisz funkcję sort(... u32, ... u32, ... u32), która rosnąco posortuje przekazane jej argumenty.

//Zad 6 Napisz funkcję swap_range(... [u32], (a1, a2), (b1, b2)), która zamieni miejscami elementy,

//Zad 7 leżące w podanych przedziałach; jeśli przedziały mają różną długość, ogranicz się do długości krótszego z nich:
