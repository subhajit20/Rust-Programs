fn main() {
    let mut my_name = String::from("Subhajit");

    // pass variable immutable references
    get_name(&my_name);


    // pass variable immutable references
    update_name(&mut my_name);

    println!("Original Owner :: {}",my_name);

    // let name1 = &my_name;
    // let name2 = &name1;
    let name3 = &my_name;
    // println!("Assign to another variable as reference : {}, {}",name1,name2);
    println!("Assign to another variable as immutable reference : {}",name3); // Subhajit Ghosh

    let name4 = &mut my_name;
    println!("Assign to another variable as mutable reference : {}",name4); // Subhajit Ghosh

    // string slicing
    let slice1 = &my_name[0..2]; // Su
    let slice2 = &my_name[2..4]; // bh
    let slice3 = &my_name[0..my_name.len()]; // Subhajit Ghosh
    println!("Slicing string value : {} {} {}",slice1,slice2,slice3);


    // string_iterator(&my_name);
}


fn get_name(name: &String){
    println!("Your Name is {}",name);
}

fn update_name(name: &mut String){
    name.push_str(" Ghosh");
    println!("Your Updated Name is {}",name);
}

fn string_iterator(name:&String){
    // convert a string into bytes array
    let bytes = name.as_bytes();
    for (i,&items) in bytes.iter().enumerate(){
        println!("Index -> {} Item -> {}",i,items);
    }
}