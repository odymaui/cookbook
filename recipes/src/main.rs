use crate::random::print_random_number;


mod random;
mod commandline;
mod ansi_term;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Running Recipes");
    
    crate::ansi_term::run_demo();

    crate::commandline::process_main();

    //random::print_random_number();
    print_random_number();

    Ok(())
}
