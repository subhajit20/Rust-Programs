#[derive(Debug)] // this is a trait which helps to print the structure with all the memebers
struct Student{
    name:String,
    roll:u32
}

// implement

impl Student{
    fn get_name(&self) -> &str {
        return &self.name
    }

    fn get_role(&self) -> &u32{
        return &self.roll
    }

    fn set_name(&mut self,updated_name:&String){
        self.name = (&updated_name).to_string();
    }
}

fn main() {
    let mut std1 = Student{
        name:String::from("Subhajit Ghosh"),
        roll:1
    };

    println!("My Name is {} Roll {}",std1.name,std1.roll); // My Name is Subhajit Ghosh Roll 1

    // if we want to print the entire instance at once
    // println!("{}",std1); // This will give an error
    // `Student` doesn't implement `std::fmt::Display
    // So we have to do like this 
    println!("{:?}",std1);
    println!("{:#?}",std1);

    // to print the referce of the structure instance we can use dbg! macro
    dbg!(&std1);

    println!("Name :: {}",std1.get_name());
    println!("Roll :: {}",std1.get_role());


    let new_student_name = String::from("Badal Ghosh");
    std1.set_name(&new_student_name);
    println!("Name :: {}",std1.get_name());
}
