pub mod FileReader {
    use std::fs;
    pub fn read_file(path: &String) -> String {
        let data: String = fs::read_to_string(path).expect("File not found!");

        return data;
    }

    pub fn arg_group<'a>(args: &'a Vec<String>) -> (&'a str, &'a str) {
        let arg_one: &str = &args[1];
        let arg_two: &str = &args[2];

        (arg_one, arg_two)
    }
}
