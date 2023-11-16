pub mod vector{
    pub mod vector;
    pub mod traits;
}
// use vector::vector::{get_vec,RUST_VEC};
use vector::traits::Student;


fn main(){
    let std_name = String::from("Subhajit");
    let std = Student::create_student(&std_name);

    println!("{}",std.firstname);
}
