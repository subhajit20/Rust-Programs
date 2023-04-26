// pub let rust_vec:Vec<String> = Vec::new();

pub static mut RUST_VEC:Vec<String> = Vec::new();

/* Accessing any global vector in that case
 * We need to use it under unsafe 
 * but this way if using it is not recommended by rust
 */
pub fn get_vec(name:&String){
    unsafe {
        RUST_VEC.push(name.to_string());
    }
}