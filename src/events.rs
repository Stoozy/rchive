use fltk::{browser::*, app::*, button::*, frame::*, window::*};
use fltk::*;
use zip::*;
use std::fs::File;

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


pub fn get_entries() -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let mut browser = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
    browser.set_filter("");
    browser.show();

    let path : std::path::PathBuf = browser.filename();
    println!("Chosen file is: {:?}", path.to_str());

    let mut file = File::open(&path).unwrap();

    let mut zipfile = zip::ZipArchive::new(file).unwrap();
    let mut iter = zipfile.file_names();
    println!("{:?}", iter.next());

    //for f in &zipfile.files{
    //    println!("{}", files.to_string());
    //}

    return ret;
}
