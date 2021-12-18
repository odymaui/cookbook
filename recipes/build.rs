use std::env;
use std::fs;
use std::path::Path;

//from //https://doc.rust-lang.org/cargo/reference/build-script-examples.html

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("Output Directory: {:?}", out_dir);
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, Build World!\"
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}