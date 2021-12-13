use crate::random::print_random_number;

mod random;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Running Recipes");
    //random::print_random_number();
    print_random_number();

    Ok(())
}
