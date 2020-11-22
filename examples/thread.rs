use std::error::Error;
use std::thread;
use std::thread::JoinHandle;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello, world!");
    }).unwrap();

    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello, world! 2");
    }).unwrap();

    thread::spawn(move || {
        println!("tez");
    }).join();
    Ok(())
}
