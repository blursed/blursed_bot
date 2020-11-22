use std::stringify;

/// designators
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
    };
}

/// overloading
macro_rules! test {
    ($left:expr; and $right:expr;) => {
        println!(
            "{:?} and {:?} = {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr;) => {
        println!(
            "{:?} or {:?} = {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// repeat
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    create_function!(macro_play);
    macro_play();

    print_result!({
        let z = 1u32;

        1 + z * 3 / 5
    });

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

    test!(true; and false;);
    test!(true; or false;);
    println!("min: {:?}", find_min!(1, 2, 3, 4, 5));
}
