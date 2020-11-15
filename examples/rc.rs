use std::rc::Rc;

/*
struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hello {}", &self.name)
    }
}

fn main() {
    let name = Rc::new("jason shin".to_string());
    let person = Person::new(name.clone()); // Clone actually increases the reference count

    person.greet();
    println!("hello again {}", name);
}
 */

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after create b {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
