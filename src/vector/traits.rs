pub struct Student<'a> {
    pub firstname:&'a String
}

impl Student<'_>{
    pub fn create_student(myname:&String) -> Student {
        let newstd = Student {
            firstname:myname
        };
    
        return newstd;
    }
}