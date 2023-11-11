// generic lifetime in struct
#[derive(Debug)]
struct Student<'s, 'n> {
    name: &'s str,
    roll: &'n u32,
}

impl<'s, 'n> Student<'s, 'n> {
    fn get_name(&self) -> &str {
        return self.name;
    }

    fn get_roll(&self) -> &u32 {
        return &self.roll;
    }
}

// generic lifetime in function
fn get_name<'s>(str1: &'s str, str2: &'s str) -> &'s str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    //generic lifetime
    let name = String::from("Subhajit");
    let roll: u32 = 10;
    let borrowed_name: &str = &name[0..3];

    let std1: Student = Student {
        name: name.as_str(),
        roll: &roll,
    };

    print!("{}\n", borrowed_name);

    print!("{:#?}", std1);

    let std_name = std1.get_name();
    let std_roll = std1.get_roll();

    print!("\n NAME :: {}\n", std_name);
    print!("\n ROLL :: {}", std_roll);

    let binding1 = String::from("Subhajit");
    let binding2 = String::from("Subhajit");
    let res = get_name(binding1.as_str(), binding2.as_str());

    print!("{}", res)
}
