#[derive(Debug)]
pub struct AuthCreds{
    pub username:String,
    pub password:String
}

impl AuthCreds{
    fn login(username:&String,password:&String){
        let new_auth = AuthCreds{
            username:username.to_string(),
            password:password.to_string()
        };
    
        println!("{:?}",new_auth);
    }

    pub fn signup(creds:AuthCreds) -> String{
        return creds.username;
    }
}

pub fn set_creds(username:&String,password:&String){
    AuthCreds::login(username,password);
}
