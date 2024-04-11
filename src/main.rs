mod lab1;
mod lab2a;
mod lab3;
mod lab4a;
mod lab2b;



// fn zad3(){
//     const N: usize = 30;
// }

fn avg(array:&[u32]) -> f32{
    let mut sum = 0;
    for i in 0..array.len(){
        sum += array[i];
    }

    sum as f32 / array.len() as f32
}

fn sort(a:&mut u32, b:&mut u32, c:&mut u32){
    if *a > *b{
        let temp = *a;
        *a = *b;
        *b = temp;
    }
    if *b > *c{
        let temp = *b;
        *b = *c;
        *c = temp;
    }
    if *a > *b{
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

// fn swap_range(array:&mut[u32], r1:(usize, usize), r2:(usize, usize)){
//     let len = r1.0-r1.1;
//     let temp:[usize;len];
//
// }
// fn wizytowka(imie: &String, nazwisko: &String) -> String{
//     return ;
// }
// fn backwards(s0:&String){
//     let mut s2 = String::new();
//
//     for c in s0.chars().rev() {
//         s2.push(c);
//     }
//     println!("{}", s2);
// }

fn main() {
    // let mut j= 0;
    // loop{
    //     j +=1;
    //     println!("{}",j);
    //     if j>5{
    //         break;
    //     }
    // }
    //lab1::info();
    //println!("{}",lab2a::met_newt_loop(0.5, 0.00001, 4));
    //println!("{}",lab2a::met_newt_recursive(0.5, 0.00001, 4, 0));
    //lab2b::is_armstrong_num(153);
    //lab2b::is_perf_num(496);
    //lab2b::prime_factors(84);
    //println!("{}",lab2b::pow_mod(5,3,13));
    // let x: i32 = 3;
    // let y: i32 = 2;
    // let z: i32 = 5;
    // println!("przed x:{x} y:{y}");
    // lab3::swap_two_args(&mut x, &mut y);
    // println!("po x:{x} y:{y}");
    //lab3::swap_sort_three_args(x, y, z);
    // let mut seed = 211537;
    // let min = 5;
    // let max = 15;
    //println!("{},{},{}", lab3::rand(&mut seed, min, max), lab3::rand(&mut seed, min, max), lab3::rand(&mut seed, min, max));
    //let mut arr: [i32;5] = [1,2,3,4,5];
    //println!("{:?}", arr);
    //lab3::swap_arr(&mut arr, 1, 5);
    //lab3::rand_perm(&mut arr, &mut seed);
    //println!("{:?}", arr);
    //let s0 = String::from("abc");
    //backwards(&s0);
    println!("sss");

}
