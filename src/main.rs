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
    lab2b::is_perf_num(496);
}
