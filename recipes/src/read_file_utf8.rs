use std::path::Path;

pub fn get_file_str() -> Result<String, String> {

    //checks at compile time.  need to have this in place or else it will fail to compile
    //macro_rules! path { () => { ".\\junk.txt" } }

    if !std::env::var("CARGO_MANIFEST_DIR").is_ok() {
        let msg = "CARGO_MANIFEST_DIR is not found!";
        println!("{}",msg.to_string());
        return Err(msg.to_string());
    }


    let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("Manifest dir string: {}", manifest_dir_string);
    let path = Path::new(&manifest_dir_string).join("junk.txt");

    if Path::new(&path).exists() {
        //let bytes = include_bytes!(path!());
        let bytes = std::fs::read(&path);
        return Ok(String::from_utf8_lossy(&bytes.unwrap()).to_string());
    } else {
        return Err(format!("{} Does Not Exist!", path.into_os_string().into_string().unwrap()));
    }
}

