use std::fs::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::path::Path;
use std::io::Write;

pub fn compress_directory() -> Result<(), std::io::Error> {
    let dir_name = "tar_test_dir";
    let bak_dir_name = "bakdir";

    if !Path::new(&dir_name).exists() {
        create_dir(&dir_name)?;
    } else {
        remove_dir_all(&dir_name)?;
        create_dir(&dir_name)?;
    }

    let mut file1 = File::create(format!("{}/{}",dir_name,"foo.txt"))?;
    let mut file2 = File::create(format!("{}/{}",dir_name,"bar.txt"))?;
    let mut file3 = File::create(format!("{}/{}",dir_name,"baz.txt"))?;
 
    file1.write(stringify!("Foo - This is a quick test.").as_bytes())?;
    file2.write(stringify!("Bar - This is a quick test.").as_bytes())?;
    file3.write(stringify!("Baz - This is a quick test.").as_bytes())?;
    
    let dir_path = format!("{}/{}", &dir_name, &bak_dir_name);

    if !Path::new(&dir_path).exists() {
        create_dir(&dir_path)?;
    }

    let tar_gz = File::create(format!("{}/{}/{}", &dir_name, &bak_dir_name, "archive.tar.gz"))?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", dir_name)?;

    //creates a copy bakdir and copy of the tar.gz but leave for now...
    println!("To extract manually - use the following to extract to the bakdir directory(the last . in the following command): tar -xf ./tar_test_dir/bakdir/archive.tar.gz -C ./tar_test_dir/bakdir");

    Ok(())
}