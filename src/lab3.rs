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

//Zadanie 4 Napisz funkcję swap_arr(arr: ..., i: usize, j: usize),
// która zamieni wartości dwóch podanych elementów pewnej tablicy:

//Zadanie 5 Stwórz funkcję rand_perm(arr: ..., seed: ...),
// permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie:
