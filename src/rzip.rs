use std::fs::File;
use std::fs;
use zip::*;
use std::io::Write;

pub fn get_file_from_path(path: String) -> File {
    let file = File::open(path).unwrap();
    return file;
}

pub fn zip_files(mut file: String, filepaths: Vec<String>, files : Vec<String>) -> (){
    // add file extension before creating
    file.push_str(".zip");
    println!("Zip file is {}", file);

    let zipfile = File::create(file).unwrap();
    let mut zip = zip::ZipWriter::new(zipfile);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);


    for i in 0..filepaths.len(){
        let cfp = &filepaths[i];
        let cf  = &files[i];
        match fs::read_to_string(cfp){
            Ok(val) => {
                println!("Writing {} to zip file", cf);
                zip.start_file(cf, options).unwrap();
                zip.write(val.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error occured: {}",e);
                println!("Path was: {}", cfp);
            }
        }
    }
    
    zip.finish().unwrap();

    println!("Successfully created zip file!");
}
