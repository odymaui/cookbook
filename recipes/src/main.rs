use atty::Stream;

#[warn(unreachable_code)]
use crate::random::print_random_number;

use crate::read_file_utf8::get_file_str;

mod random;
mod commandline;
mod ansi_term;
mod compression;
mod concurrency;
mod external_commands;
mod type_stuff;
mod misc;
mod macro_example;
mod sort;
mod make_change;
mod format;
mod read_file_utf8;

//this is generated from the build.rs 
//https://doc.rust-lang.org/cargo/reference/build-script-examples.html
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Running Recipes");

    let rslt = get_file_str();

    match rslt {
        Ok(r) => println!("Result: {}", r),
        Err(r) => println!("Error: {}", r)
    }

    format::print_formatted_stuff();

    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
      println!("I'm not in a terminal");
    }

    let items1 = &mut vec!(5,3,5,7,6,1,2,4,3,8,1,0);
    let items2 = &mut vec!(-5,-3,-5,-7,-6,-1,-2,-4,-3,-8,-1,0);
    let items3 = &mut vec!(-5.7,-3.8,-5.0,-7.0,-6.0,-1.0,-2.0,-4.0,-3.0,-8.0,-1.0,0.0);

    sort_arrays(items1);
    sort_arrays(items2);
    sort_arrays(items3);

    macro_example::run_min_calls();

    misc::print_ruler();
    misc::simple_cast();
    misc::simple_type_alias();
    misc::get_stringify_result();
    misc::run_fibonacci_sequence(10);
    misc::iterator_processing();
    //below...
    self::run_fibonacci_sequence_stuff();

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

    crate::make_change::make_change(447);
    crate::make_change::make_change(777);
    crate::make_change::make_change(783);
    
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

fn sort_arrays<T: std::marker::Copy + std::fmt::Display + std::cmp::PartialOrd>(array: &mut Vec<T>) {
    print!("Before: [ ");
    for i in array.into_iter() {
        print!("{} ", i);
    }
    println!("]");

    sort::merge_sort(array);
    
    print!("After: [ ");
    for i in array.into_iter() {
        print!("{} ", i);
    }
    println!("]");
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


fn run_fibonacci_sequence_stuff() {
    
    //this will be an infinite loop???
    //stops due to overflow if no break...
    for val in misc::get_fibonacci_sequence() {
        //debug prints what are private fields
        //33 |         println!("External val: {}", val.current);
        //   |                                 ^^^^^^^ private field
        println!("External val: {:?}", val);
        println!("External val: {} -> {}", val.index, val.current);
        if val.index > 91 { break; }
    }

    let idx = 7;
    for v in misc::get_fibonacci_sequence().take(idx) {
        println!("{}->{}", v.index, v.current);    
    }
    println!("sum of the first {}: {}", idx, misc::get_fibonacci_sequence().take(idx).map(|f| f.current).sum::<u64>());
    //don't need the index since it's available
    //Like most indexing operations, the count starts from zero, so nth(0) returns the first value, nth(1) the second, and so on.
    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
    //set idx + 1 since 0 based...
    println!("The {}th: {}", idx + 1, misc::get_fibonacci_sequence().nth(idx).unwrap().current);
    println!("{}", format!("The {}th: {}", idx + 1, misc::get_fibonacci_sequence().nth(idx).unwrap()));

    //need to pick to_string or fmt...
    println!("default display {}", misc::get_fibonacci_sequence().nth(idx).unwrap());

    let nothing = misc::get_fibonacci_sequence().map(|f| f.current);

    println!("Nothing: {:?}", nothing);
    //doesn't work come back later...
    /*
    let something: dyn std::iter::Iterator<Item=misc::Fibonacci> = misc::get_fibonacci_sequence().map(|f| f).take(50).collect();

    for some in &mut something {
        println!("some of something: {}", some);
    }
    */
}