#[derive(Debug)]
enum RecursiveList {
    Cons(i32, Box<RecursiveList>), // Cons represents a value and a reference to the next element
    Nil,                           // Nil represents the end of the list
}

use RecursiveList::{Cons, Nil};

fn main() {
    println!("Hello, world!");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // print!("{:#?}", list);

    // match list {
    //     Cons(a1, a2) => {
    //         print!("{}", a1);
    //         print!("{:#?}", a2);

    //         match *a2 {
    //             // here use ' * ' because this element is containing a reference to the next element in the list
    //             // so we have to dereference that element, thats why we use *
    //             Cons(l1, l2) => {
    //                 print!("{}\n", l1);
    //                 print!("{:#?}\n", l2);
    //             }
    //             Nil => todo!(),
    //         }
    //     }
    //     Nil => todo!(),
    // }

    let mut current = &list;

    loop {
        match current {
            Cons(value, next) => {
                print!("{} \n", value);

                current = next
            }
            Nil => break,
        }
    }

    while let Cons(value, next) = current {
        println!("{} \n", value);

        current = next;
    }
}
