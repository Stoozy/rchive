use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use fltk::{browser::*, app::*, button::*, frame::*, window::*};
use fltk::*;
use zip::*;


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

pub fn unzip(filename : String, dir_path : PathBuf){
    let zipfile = get_file_from_path(filename);
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    zip.extract(dir_path).unwrap();
}


pub fn get_entries() -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let mut browser = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
    browser.set_filter("");
    browser.show();

    let path : std::path::PathBuf = browser.filename();
    println!("Chosen file is: {:?}", path.to_str());

    let mut file = File::open(&path).unwrap();

    let mut zipfile = zip::ZipArchive::new(file).unwrap();
    //let mut iter = zipfile.file_names();
    for i in 0..zipfile.len() {
        ret.push(zipfile.by_index(i).unwrap().name().to_string());
    }

    return ret;
}

pub fn file_new_handler(){
    println!("Test");
}

pub fn has_dir(s : String) -> bool {
    for c in s.chars(){
        if c == '/' {
            return true;
        }
    }
    return false;
}

