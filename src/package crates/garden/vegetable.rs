fn outside_module(){
    println!("This function is outside of the module");
}

pub mod vegetable{
    /**
     * this is a way on how we can call any
     * function outside of an module from
     * the function inside of an module
     */
    // use crate::garden::vegetable::outside_module;

    pub fn hellow(){
        println!("Hello from vegetable file...");

        #[doc = r"* this is another way on how we can call any
        * function outside of an module from
        * the function inside of an module"]
        super::outside_module();

        #[doc = r"* this is the way on how we can call any
        * function inside of an module from
        * another function inside the same module"]
        self::inside_module()
    }

    pub fn inside_module(){
        println!("This is inside module function from vegetable module ...")
    }
}