struct Cat {
    name: String,
}

fn test<T>(t: T)
where
    T: Copy,
{
}

fn main() {
    println!("test!");
    let cat = Cat {
        name: String::from("some cat"),
    };
    test(&cat);
    test(&cat)
}
