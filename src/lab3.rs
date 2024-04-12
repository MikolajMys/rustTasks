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
    *seed = (80 * *seed + 70) % 88553;
    //println!("{}", *seed);
    *seed % (max_rand-min_rand+1) + min_rand
}
//Zadanie 4 Napisz funkcję swap_arr(arr: ..., i: usize, j: usize),
// która zamieni wartości dwóch podanych elementów pewnej tablicy:
pub fn swap_arr(arr: &mut [i32], i: usize, j: usize){
    let temp: i32 = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

//Zadanie 5 Stwórz funkcję rand_perm(arr: ..., seed: ...),
// permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie:
pub fn rand_perm(arr: &mut [i32], seed: &mut i128){
    for i in 0..arr.len() {
        let j = rand(seed, 0, (arr.len() - 1) as i128) as usize;
        swap_arr(arr, i, j);
        println!("{} - {}",i,j);
    }
}

//Inna wersja zadań:

//Zad 1 Napisz funkcję d2((x, y), (x, y)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^2:
fn d2(x1:(i32, i32), x2:(i32, i32)) -> f32{
    let a:f32 = (x1.0 - x2.0) as f32;
    let b:f32 = (x1.1 - x2.1) as f32;
    let sum:f32 = a*a + b*b;
    sum.sqrt()
}
//Zad 2 Napisz funkcję d3((x, y, z), (x, y, z)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^3:
fn d3(x1:(i32, i32, i32), x2:(i32, i32, i32)) -> f32{
    let a:f32 = (x1.0 - x2.0) as f32;
    let b:f32 = (x1.1 - x2.1) as f32;
    let c:f32 = (x1.2 - x2.2) as f32;
    let sum:f32 = a*a + b*b + c*c;
    sum.sqrt()
}
//Zad 3 Stwórz tablicę N elementów, którą wypełnisz resztami z dzielenia liczby 100 przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości tablicy od końca:
fn division_rest(arr:&mut[i32], n:usize){

//println!("{:?}", arr);
    for i in 1..=n{
        arr[i-1] = 100 % i as i32;
    }
//println!("{:?}", arr);
// for i in 1..=n{
// println!("{}", arr[n - i]);
// }
    for &i in arr.iter().rev(){
//println!("{}",i);
    }
}
//Zad 4 Napisz funkcję avg(&[u32]) -> f32, która obliczy średnią arytmetyczną liczb z tablicy:
fn avg(array:&[u32]) -> f32{
    let n = array.len();
//let sum: u32 = array.iter().sum();
//sum as f32 / n as f32
    let mut sum: u32 = 0;
    for i in 0..n{
        sum += array[i];
    }
    sum as f32 / n as f32
}
//Zad 5 Napisz funkcję sort(... u32, ... u32, ... u32), która rosnąco posortuje przekazane jej argumenty:
fn sort(a:&mut u32, b:&mut u32, c:&mut u32){
// println!("{a},{b},{c}");
// if a > b {
// let temp: u32 = *a;
// *a = *b;
// *b = temp;
// }
// if b > c {
// let temp: u32 = *b;
// *b = *c;
// *c = temp;
// }
// if a > b {
// let temp: u32 = *a;
// *a = *b;
// *b = temp;
// }
// println!("{a},{b},{c}");
    let mut vec: Vec<u32> = Vec::new();
    vec.push(*a);
    vec.push(*b);
    vec.push(*c);
//println!("{:?}",vec);
    vec.sort();
//println!("{:?}",vec);
}
//Zad 6 Napisz funkcję swap_range(... [u32], (a1, a2), (b1, b2)), która zamieni miejscami elementy,
// leżące w podanych przedziałach; jeśli przedziały mają różną długość, ogranicz się do długości krótszego z nich:
fn swap_range(array:&mut [u32], x1:(usize,usize), x2:(usize,usize)){
    let mut range1 = x1;
    let mut range2 = x2;

    let size1 = x1.1 - x1.0;
    let size2 = x2.1 - x2.0;

    if size1 > size2{
        range1.1 = range1.0 + size2;
    } else if size2 > size1{
        range2.1 = range2.0 + size1;
    }

    let mut j = 0;
    println!("{:?}",array);
    for i in range1.0..=range1.1{
        let temp = array[i];
        array[i] = array[range2.0 + j];
        array[range2.0 + j] = temp;
        j += 1;
    }
    println!("{:?}",array);
}
// fn main() {
//     let res:f32 = d2((2,0),(4,0));
// //println!("{}", res);
//     let res:f32 = d3((0,0,0),(0,2,0));
// //println!("{}", res);
//
//     let mut arr:[i32;30] = [0;30];
//     division_rest(&mut arr, 30);
//     let mut array:[u32;6] = [1,2,3,4,5,6];
//     let avg = avg(&array);
// //println!("{avg}");
//     let mut a:u32 = 3;
//     let mut b:u32 = 2;
//     let mut c:u32 = 1;
//     sort(&mut a,&mut b,&mut c);
//     swap_range(&mut array,(0,2),(4,5))
// }