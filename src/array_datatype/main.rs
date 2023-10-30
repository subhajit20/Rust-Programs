use std::io;

fn main() {
    // Array data type
    let numbers:[u32;10] = [1,2,3,4,5,6,7,8,9,10];

    let mut guess_index = String::new();

    println!("Enter a number :: ");
    io::stdin()
        .read_line(&mut guess_index)
        .expect("Enter the string properly...");

    
    let guess_index:usize = guess_index.trim().parse().expect("Input should be number");


    println!("{}",numbers[guess_index]);
    println!("The number is  {guess_index}");
}
