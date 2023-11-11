pub fn hello(){
    println!("Helloooo from module file...")
}

pub struct Student{
    pub name:String,
    pub roll:u32
}

impl Student{
    pub fn get_name(&self) -> &String{
        println!("{}",self.name);
        return &self.name
    }

    pub fn get_roll(&self) -> &u32{
        println!("{}",self.roll);
        return &self.roll
    }
}