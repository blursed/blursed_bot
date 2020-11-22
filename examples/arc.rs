use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let foo = Arc::new(vec![1]);
    let bar = foo.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("{:?} test", &bar);
    });

    println!("{:?}", &foo)
}
