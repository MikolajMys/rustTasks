//Zad1
pub fn find(array:&mut[u32;9]) -> u32{
    //array.sort();
    let mut a:[u32;10] = [0;10];
    let mut v:Vec<u32> = Vec::new();
    //println!("{:?}", array);
    for i in 0..9{
        a[array[i] as usize] += 1;
    }
    for j in 0..9{
        if a[j] == 0{
            v.push(j as u32);
        }
    }
    v.sort();
    //println!("{:?}", a); //count array
    v[0]
}
//Zad2
pub fn pierwiastki(a: f32, b: f32, c:f32) -> u8{
    let delta = b*b - (4.0*a*c);
    let sqrdel = delta.sqrt();
    let mut sum:u8 = 0;
    let x1 = -b-sqrdel/2.0*a;
    let x2 = -b+sqrdel/2.0*a;
    //println!("{},{}", x1,x2);
    if x1 >= 0.0{
        sum += 1;
    }
    if x2 >= 0.0{
        sum += 1;
    }
    sum
}
//Zad3
pub fn reverse(arr:&mut[u32]){
    let mut v:Vec<u32> = Vec::new();
    for &i in arr.iter().rev(){
        v.push(i);
    }
    for i in 0..arr.len(){
        arr[i] = v[i];
    }
}

// fn main() {
//     println!("Zad1:");
//     let mut array1:[u32;9] = [5,8,2,0,6,4,7,1,9];
//     let x1:u32 = find(&mut array1);
//     println!("dla {:?} -> {}",array1,x1);
//     let mut array2:[u32;9] = [5,8,8,0,6,4,7,8,9];
//     let x2:u32 = find(&mut array2);
//     println!("dla {:?} -> {}",array2,x2);
// //====================================================
//     println!("Zad2:");
//     let z:u8 = pierwiastki(2.0,5.0,0.0);
//     println!("{z}");
// //====================================================
//     println!("Zad3:");
//     let mut arr1:[u32;3] = [7,1,2];
//     let mut arr2:[u32;5] = [5,4,3,2,1];
//     println!("{:?}", arr1);
//     reverse(&mut arr1);
//     println!("{:?}", arr1);
//     println!("{:?}", arr2);
//     reverse(&mut arr2);
//     println!("{:?}", arr2);
// }
