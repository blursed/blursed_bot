use std::stringify;

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    }
}

fn main() {
    create_function!(macro_play);
    macro_play();

    print_result!({
        let z = 1u32;

        1 + z * 3 / 5
    })

    /// There are more designators
    /// block
    // expr is used for expressions
    // ident is used for variable/function names
    // item
    // literal is used for literal constants
    // pat (pattern)
    // path
    // stmt (statement)
    // tt (token tree)
    // ty (type)
    // vis (visibility qualifier)
}
