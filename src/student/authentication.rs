#[derive(Debug)]
struct AuthCreds{
    username:String,
    password:String
}

impl AuthCreds{
    fn login(username:&String,password:&String){
        let new_auth = AuthCreds{
            username:username.to_string(),
            password:password.to_string()
        };
    
        println!("{:?}",new_auth);
    }

}

pub fn set_creds(username:&String,password:&String){
    AuthCreds::login(username,password);
}
