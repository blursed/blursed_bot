use std::rc::Rc;

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
