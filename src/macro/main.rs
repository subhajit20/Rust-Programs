macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for creating variable/function names.
    ($func_name:ident) => {
        fn $func_name(name: String) {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {}", name);
        }
    };
}

macro_rules! create_variable {
    ($variable_name:ident,$func_name:ident,$var_type:ty,$var_value:expr) => {
        let $variable_name: $var_type = $var_value;

        let $func_name = |varname: $var_type| -> $var_type { varname };
    };
}

create_function!(foo);
create_function!(boo);

fn main() {
    foo("Subhajit".to_string());
    boo("Subhajit".to_string());

    // creating variable using macro!
    create_variable!(p, get_p, u32, 10);
    println!("Variable P is -> {}", get_p(p));
}
