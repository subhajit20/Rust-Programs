// pub mod vector{
//     pub mod vector;
//     pub mod traits;
// }
// // use vector::vector::{get_vec,RUST_VEC};
// use vector::traits::Student;

fn main(){

    // Lifetime concept

    // This example gives an error 
    // let y:&u32;           // + ------ 'b - | 
    // {                     // + -----  'b   |
    //     let x:u32 = 20;  //                |
    //     y = &x;          // - ------  'b   |
    // }                   //                 |
    //                     //                 |
    // println!("{y}");    // - -------- 'b - |

    let mut name = String::from("I am");
    // let n1 = student1(&name);
    let n2 = student2(&mut name);

    // println!("{n1}");
    println!("{n2}");
}

// Define life with 'a or 'any character
// Using life time means   
// explicitly declare the life time of variable

// a reference with an explicit lifetime
fn student1<'a>(x:&'a String) -> &'a String{
    return x;
}

// a mutable reference with an explicit lifetime
fn student2<'a>(x:&'a mut String) -> &'a String{
    x.push_str(" Subhajit");

    return x;
}