// Making structure
struct Student{
    name:String
}

// main function which will be executed duting runtime
fn main() {
    // let mut name = String::from("Subhajit");
    // println!("{name}");
    let mut std1 = Student{
        name:String::from("Subhajit")
    };
    println!("{}",std1.name);
    append_surname(&mut std1.name);
}

/*
 * Normal Fuction
 */
fn append_surname(name:&mut String) -> String {
    name.push_str(" Ghosh");
    println!("Hello {name}");

    return name.to_string();
}
