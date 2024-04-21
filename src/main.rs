mod lab1;
mod lab2a;
mod lab2b;
mod lab3;
mod lab4a;
mod lab4b;
mod kolokwium1;
mod lab6a;


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
    let mut seed = 211537;
    // let min = 5;
    // let max = 15;
    //println!("{},{},{}", lab3::rand(&mut seed, min, max), lab3::rand(&mut seed, min, max), lab3::rand(&mut seed, min, max));
    let mut arr: [i32;5] = [1,2,3,4,5];
    //println!("{:?}", arr);
    //lab3::swap_arr(&mut arr, 1, 5);
    //lab3::rand_perm(&mut arr, &mut seed);
    //println!("{:?}", arr);
    // let s0 = String::from("abcdef");
    // let s1 = lab4b::backwards(&s0);
    // println!("{s1}");
    let  n = 12345;
    let x = lab2b::met_newt_loop(lab2b::f, lab2b::fp, 0.5, 0.00001, 4);
    //println!("{x}");
    let mut array1:[u32;9] = [5,8,2,0,6,4,7,1,9];
    let x1:u32 = kolokwium1::find(&mut array1);
    //println!("dla {:?} -> {}",array1,x1);
    let mut arr1:[u32;3] = [7,1,2];
    //println!("{:?}", arr1);
    kolokwium1::reverse(&mut arr1);
    //println!("{:?}", arr1);
    //println!("{}", lab4a::liczba_wystapien("bambusowaća", 'a'));
    let s0 = lab4b::co_drugi_znak("n1a2p3i4s".to_string());
    println!("{s0}");
}
