pub mod garden{
    pub mod vegetable;
}
pub mod module; // converting a file into module

use module::Student; // use the above as a crate
use garden::vegetable::vegetable::hellow;


fn main() {
    println!("Hello, world!");
    module::hello();
    let std = Student{
        name:String::from("Subhajit Ghosh"),
        roll:1
    };
    hellow();
    std.get_name();
    std.get_roll();
}
