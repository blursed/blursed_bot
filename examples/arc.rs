use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let foo = Arc::new(vec![1]);
    let bar = foo.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("{:?} test", &bar);
    })
    .join();
    println!("{:?}", foo);
    Ok(())
}
