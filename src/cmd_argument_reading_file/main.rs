// making a command line arguement project using rust
use std::env::args; // this library is used for getting command line arguements

pub mod reading_file {
    pub mod FileReader;
}

use reading_file::FileReader::FileReader::{arg_group, read_file};

fn main() {
    let args: Vec<String> = args().collect();

    let (res1, res2) = arg_group(&args);
    print!("{} \n", res1);
    print!("{} \n", res2);
    // run cargo run -- arg1 arg2

    /* dbg!(args);  [src/main.rs:8] args = [
        "target/debug/minigrep",
        "arg1",
        "arg2",
    ] */
    // let arg1 = &args[1];
    // let arg2 = &args[2];

    // print!("Arguement 1 :: {} \n", arg1);
    // print!("Arguement 1 :: {} \n", arg2);

    // let file_path = String::from("hello.txt");
    // let file_data: String = read_file(&file_path);

    // print!("{}", file_data);

    // let chunk: &str = &file_data[0..9];
    // print!("{}", chunk)
}
