use std::fs::*;
use flate2::Compression;
use flate2::write::GzEncoder;
//care.... https://stackoverflow.com/questions/68395708/compressing-and-decompressing-a-tar-gz-file
use flate2::read::GzDecoder;
use std::path::Path;
use std::io::Write;
use tar::Archive;

struct CompressionValues<'a> {
    dir_name: &'a str,
    dir_path: String,
    tar_file_name: String,
    extract_dir: String
}

impl<'a> Default for CompressionValues<'a> {
    fn default<'b>() -> CompressionValues<'a> {

        let dir_name = "tar_test_dir";
        let bak_dir_name = "bakdir";
        let tar_file = "archive.tar.gz";
        let dir_path = format!("{}/{}", dir_name, bak_dir_name);
        let extract_dir_name = "extraction";
    
        let tar_file_name = format!("{}/{}/{}", dir_name, bak_dir_name, tar_file);
        let extract_dir = format!("{}/{}/{}", dir_name, bak_dir_name, extract_dir_name);

        CompressionValues {
            dir_name: dir_name,
            dir_path: dir_path,
        
            tar_file_name: tar_file_name,
            extract_dir: extract_dir
        }
    }
}


pub fn create_compress_directory() -> Result<(), std::io::Error> {

    let config = CompressionValues::default();

    if !Path::new(&config.dir_name).exists() {
        create_dir(&config.dir_name)?;
    } else {
        remove_dir_all(&config.dir_name)?;
        create_dir(&config.dir_name)?;
    }

    let mut file1 = File::create(format!("{}/{}",&config.dir_name,"foo.txt"))?;
    let mut file2 = File::create(format!("{}/{}",&config.dir_name,"bar.txt"))?;
    let mut file3 = File::create(format!("{}/{}",&config.dir_name,"baz.txt"))?;
 
    file1.write(stringify!("Foo - This is a quick test.").as_bytes())?;
    file2.write(stringify!("Bar - This is a quick test.").as_bytes())?;
    file3.write(stringify!("Baz - This is a quick test.").as_bytes())?;
  
    if !Path::new(&config.dir_path).exists() {
        create_dir(&config.dir_path)?;
    }

    let tar_gz = File::create(&config.tar_file_name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", &config.dir_name)?;

    //creates a copy bakdir and copy of the tar.gz but leave for now...
    println!("To extract manually - use the following to extract to the bakdir directory(the last . in the following command): tar -xf ./tar_test_dir/bakdir/archive.tar.gz -C ./tar_test_dir/bakdir");

    Ok(())
}


pub fn decompress_tar() -> Result<(), std::io::Error> {

    let config = CompressionValues::default();

    if !Path::new(&config.tar_file_name).exists() {
        create_compress_directory()?;
    }

    if !Path::new(&config.extract_dir).exists() {
        create_dir(&config.extract_dir)?;
    } else {
        remove_dir_all(&config.extract_dir)?;
        create_dir(&config.extract_dir)?;
    }

    let tar_gz = File::open(&config.tar_file_name)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&config.extract_dir)?;

    Ok(())
}

