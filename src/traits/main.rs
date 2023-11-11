// trait is as same as the interface is in other languages
// but it has some differences

trait StudentInfo {
    fn get_name(&self) -> &String;

    fn name(&self) -> &str;
}

struct Student {
    name: String,
    roll: usize,
}

impl StudentInfo for Student {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn name(&self) -> &str {
        return &self.name;
    }
}

fn main() {
    let std = Student {
        name: String::from("Subhajit Ghosh"),
        roll: 1,
    };

    let std_name = std.get_name();
    let name = std.name();

    println!("{}", name);
}
