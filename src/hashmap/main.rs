use std::collections::HashMap;
pub mod package{
    pub mod HashMap;
}
use crate::package::HashMap::StudentHashMap::Student;

fn main() {
    // Declaring hashmap with type
    let mut student_map:HashMap<u32,&Student> = HashMap::new();

    let std_1:Student = Student{
        name:Some(String::from("Subhajit Ghosh")),
        roll:None
    };

    student_map.insert(1,&std_1); // updating hashmap data using insert function
    println!("{:#?}",student_map);

    let std_1_hash = student_map.get(&1).copied(); // accessing a key from the hashmap

    println!("{:#?}",std_1_hash);
    

    for (key,value) in &student_map{
        println!("Key {} \nValue {:#?}",key,value);
    }
}
