use rand::prelude::*;

trait Animal {
    fn talk(&self);
}

struct Cat {}
struct Dog {}

impl Animal for Cat {
    fn talk(&self) {
        println!("meow!")
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark")
    }
}

// https://medium.com/@vikram.fugro/dyn-impl-and-trait-objects-rust-fd7280521bea
fn animal_talk(a: &dyn Animal) {
    a.talk()
}

fn animal() -> Box<dyn Animal> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(1.0 / 3.0) {
        return Box::new(Dog {});
    }

    return Box::new(Cat {});
}

fn main() {
    let c = Cat {};
    let d = Dog {};

    animal_talk(&c);
    animal_talk(&d);
}
