#[derive(Debug)]
struct StudentInfo{
    name: String,
    roll: u32,
    qualified:bool
}

// #[derive(Debug)]
// enum Biryani{
//     Chicken(String),
//     Mutton(String)
// }

#[derive(Debug)]
enum Student{
    pass(StudentInfo),
    fail(StudentInfo)
}

fn main() {
    // let chicken_biryani = Biryani::Chicken(String::from("Chicken Biryani"));
    // let mutton_biryani = Biryani::Mutton(String::from("Mutton Biryani"));

    // println!("{:?}",chicken_biryani);
    // println!("{:?}",mutton_biryani);
    // let std = StudentInfo{
    //         name:String::from("Subhajit Ghosh"),
    //         roll:1,
    //         qualified:true
    //     };
    // let std_array = [std];

    // println!("{}",std_array[0].name);

    let std_1 = Student::pass(
        StudentInfo{
            name:String::from("Subhajit Ghosh"),
            roll:1,
            qualified:true
        }
    );

    let std_2 = Student::fail(
        StudentInfo{
            name:String::from("Badal Ghosh"),
            roll:2,
            qualified:false
        }
    );

    println!("Passed :: {:#?}",std_1);
    println!("Passed :: {:#?}",std_2);
    

    if let Student::fail(info) = std_2 {
        println!("Passed :: Name: {}, Roll: {}, Qualified: {}", info.name, info.roll, info.qualified);
    }else{
        println!("Student is passed!");
    }
}
