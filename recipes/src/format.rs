/*
fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
} 
*/
//https://doc.rust-lang.org/std/fmt/
pub fn print_formatted_stuff() {

    //numbers or positional 
    println!("{}", format!("{} {1} {2} {} {0} {}", 1, 2, 3)); // => "1 2 3 2 1 3"
    //named parameter
    println!("{foo} {bar} {baz}", foo="bar", bar="foo", baz="foobar"); // => "bar foo foobar"

    /* 
    todo...
    just not working even with the latest edition...
    error: there is no argument named `b`
 --> src\format.rs:2:14
  |
2 |     format!("{b} {a}")
  |              ^^^
  */
    
    //let formatted_val = make_string(567, "label:");

    //won't work but does in the playground
    //println!("{}", formatted_val);

    /*
    fn print(foo1: &str, bar1: &str) -> String {
        format!("{foo1} {bar1}") 
    }

    println!("{}", print("1foo", "1bar"));
    
    let argument = 2 + 2;
    println!("{}", format!("{argument}"));   // => "4"


    println!("{}", make_string(927, "label")); // => "label 927"

    */

    /*
    
    [fill]< - the argument is left-aligned in width columns
    [fill]^ - the argument is center-aligned in width columns
    [fill]> - the argument is right-aligned in width columns

    */

    assert_eq!(format!("Hello {:_<5}!", "x"),  "Hello x____!");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:_^5}!", "x"),  "Hello __x__!");
    assert_eq!(format!("Hello {:_>5}!", "x"),  "Hello ____x!");

    //note default is a space...
    assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!");

    //todo finish numeric and other options...
}

