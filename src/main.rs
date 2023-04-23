pub mod student{
    pub mod authentication;
    // pub mod student;
    // pub mod info;
}

// use crate::student::student::student::hello;
// use crate::student::info::my_info::getinfo;
use crate::student::authentication::set_creds;

fn main() {
    let username = String::from("Subhajit");
    let password = String::from("SUBHAJII");
    set_creds(&username,&password);
}


