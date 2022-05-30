use std::path::Path;

pub fn get_file_str() -> Result<String, String> {

    //checks at compile time.  need to have this in place or else it will fail to compile
    macro_rules! path { () => { "..\\junk.txt" } }

    if Path::new(path!()).exists() {
        let bytes = include_bytes!(path!());
        return Ok(String::from_utf8_lossy(bytes).to_string());
    } else {
        return Err(format!("{} Does Not Exist!", path!()));
    }
}

