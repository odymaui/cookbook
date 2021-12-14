use ansi_term::Style;
use ansi_term::Colour::{Blue, Cyan, Yellow, RGB};

pub fn run_demo() {
    //https://docs.rs/ansi_term/latest/ansi_term/
    println!("Yellow on blue: {}", Style::new().on(Blue).fg(Yellow).paint("yow!"));
    println!("Also yellow on blue: {}", Cyan.bold().underline().on(Blue).fg(Yellow).paint("zow!"));


    println!("{}",RGB(10, 10, 10).on(Yellow).paint("kind of light grey on yellow"));
    println!("{}",RGB(255, 255, 255).on(Cyan).paint("white on cyan"));
}