#[warn(unreachable_code)]

use crate::random::print_random_number;

mod random;
mod commandline;
mod ansi_term;
mod compression;
mod concurrency;
mod external_commands;
mod type_stuff;
mod misc;
mod macro_example;

//this is generated from the build.rs 
//https://doc.rust-lang.org/cargo/reference/build-script-examples.html
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Running Recipes");

    macro_example::run_min_calls();

    misc::simple_cast();
    misc::simple_type_alias();

    println!("Build Result Message: {}", message());

    println!("Running Without Joining");
    crate::concurrency::run_threads_without_with_join(false);
    println!("Running With Joining");
    crate::concurrency::run_threads_without_with_join(true);
    crate::concurrency::run_thread_move_data();
    crate::concurrency::thread_channel_example();
    println!("______________________________________________\n");
    crate::concurrency::thread_channel_example_multiple_producers();

    crate::concurrency::mutex_demo();

    crate::external_commands::run_os_command();

    crate::type_stuff::example_type_test();

    crate::type_stuff::count_shapes();
    
    //just return for now...
    return Ok(());

    #[allow(unreachable_code)]
    {
    simple_sort();

    crate::compression::create_compress_directory()?;
    crate::compression::decompress_tar()?;
    crate::ansi_term::run_demo();
    crate::commandline::process_main();

    //random::print_random_number();
    print_random_number();

    Ok(())
    }
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
