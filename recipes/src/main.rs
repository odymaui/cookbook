use crate::random::print_random_number;


mod random;
mod commandline;
mod ansi_term;
mod compression;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Running Recipes");

    simple_sort();

    crate::compression::compress_directory()?;
    crate::ansi_term::run_demo();
    crate::commandline::process_main();

    //random::print_random_number();
    print_random_number();

    Ok(())
}

fn simple_sort() {

    println!("Sort A Vector: Before");
    let mut items = vec!("foo", "bar", "baz");

    for i in &items {
        println!("{}", i);
    }

    println!("After Vector:");

    items.sort();

    for i in items {
        println!("{}", i);
    }

}
