/*
The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator:

This is a list of all the designators:

    block
    expr is used for expressions
    ident is used for variable/function names
    item
    pat (pattern)
    path
    stmt (statement)
    tt (token tree)
    ty (type)


To repeat you can use ,* or ,+ for 0 or 1 or more items.

*/

// min! will calculate the minimum of any number of arguments
macro_rules! min_value {
    // base case
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // call min! on the tail `$y`
        std::cmp::min($x, min_value!($($y),+))
    )
}


pub fn run_min_calls() {

    println!("{}", min_value!(1u32));
    println!("{}", min_value!(1u32 + 2 , 2u32));
    println!("{}", min_value!(5u32, 2u32 * 3, 4u32));
    println!("{}", min_value!(5u32, 2u32 * 3, 4u32, 1u32, 0u32));

}

#[test]
fn run_min_macro() {
    assert!(min_value!(1u32) == 1);
    assert!(min_value!(5u32, 2u32 * 3, 4u32, 1u32, 0u32) == 0);
}