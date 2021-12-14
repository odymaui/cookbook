use clap::{Arg, App};

pub fn example_cmd_line_matches<'a>() -> clap::App<'a, 'a> {
    let app_parser = App::new("Sample App Parser")
        .version("0.0.1")
        .author("Foo Bar")
        .about("Example Cmd Line Parsing")
        .arg(Arg::with_name("foo")
                .short("f")
                .long("foo_")
                .takes_value(true)
                .help("The First Parameter Foo is a file"))
        .arg(Arg::with_name("bar")
                .short("b")
                .long("bar_")
                .takes_value(true)
                .help("The Second Parameter Bar is a number"));

    app_parser
}

pub fn process_results(matches: clap::ArgMatches) {

    let first_param_foo = matches.value_of("foo").unwrap_or("He Buddy, Pass A Value!!!");
    println!("The file passed is: {}", first_param_foo);

    let num_str = matches.value_of("bar");
    match num_str {
        None => println!("No idea what the number is, nothing passed for bar!"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Bar is {}.", n),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }

}

pub fn process_main() {

    println!("processing args...");

    let app_parser = example_cmd_line_matches();

    //get_matches will work off of the actual passed values
    //this will simulate it based on items -- application name is always the first value
    
    for (idx, p) in std::env::args().enumerate() {
        println!("{}->{}", idx, p);
    }
    let r = app_parser.get_matches_from(vec!("app_name", "-f", "foo_value", "-b", "5"));
    process_results(r);
}

