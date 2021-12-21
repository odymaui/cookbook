

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