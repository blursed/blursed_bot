/*
fn some_func<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    y
}

fn main() {
    let x = 12;
    let y = 13;
    println!("{}", some_func(&x, &y))
}
*/

/*
fn some_func1<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    y
} // joint lifetime between x and y
*/

fn some_func2<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    y
} // meant to be a disjoint lifetime but still crashes

fn some_func(x: &i32) -> &i32 {
    // lifetime elision
    x
}

fn main() {
    let x = 12;
    let v = {
        let y = 13;
        // y is a borrowed value that does not live longer than x
        // let z = some_func(&x, &y); // doesn't error because lifetime of y finishes at this point
        some_func2(&x, &y)
    };
}
