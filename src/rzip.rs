use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use fltk::{browser::*, app::*, button::*, frame::*, window::*};
use fltk::*;
use zip::*;


//pub enum DirEntry {
//    Dir(Vec<DirEntry>),
//    Files(Vec<PathBuf>)
//}


#[derive(Debug, Clone)]
pub struct DirEntry{
    cdir: String,
    pub dirs: Vec<DirEntry>,
    pub files: Vec<String>,
}

impl DirEntry {
    pub fn get_name(&self) -> String {
        return self.cdir.clone();
    }
    pub fn get_dirs(&self) -> Vec<DirEntry> {
        return self.dirs.clone();
    }
    pub fn get_files(&self) -> Vec<String>{
        return self.files.clone();
    }

    pub fn contains_dir(&self, find : String) -> bool {
        for dir in &self.dirs {
            if find == dir.cdir {
                return true;
            }
        }
        return false;
    }

    // Make sure to check if contains dir before doing this
    pub fn find_child_dir(&self, find: String) -> usize {
        for i in 0..self.dirs.len() {
            if find == self.dirs[i].get_name() {
                return i;
            }
        }
        return 0;
    }

    pub fn add_dir(& mut self,  mut new_dir : DirEntry) -> usize {
        let mut idx : usize = 0;
        let name = new_dir.get_name();
        self.dirs.push(new_dir);
        for i in 0..self.dirs.len() {
            if self.dirs[i].get_name() == name {
                idx = i;
            }
        }
        return idx;
    }

    pub fn add_file(& mut self, file : String ) -> () {
        self.files.push(file);
    }
    

}


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


pub fn get_entries() ->  DirEntry {
    let mut ret: DirEntry =  DirEntry{cdir: String::from("/"), dirs: Vec::new(), files: Vec::new() };

    let mut browser = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
    browser.set_filter("");
    browser.show();

    // TODO: check file ext for compatibility (.zip only for now)

    let path : std::path::PathBuf = browser.filename();
    println!("Chosen file is: {:?}", path.to_str());

    let mut file = File::open(&path).unwrap();
    let mut zipfile = zip::ZipArchive::new(file).unwrap();

    let mut files = zipfile.file_names();


    // add dir if not found
    // continue until parent dir of file is found
    // add file to parent dirs files
    // repeat for every file

    for file in files {
        // has folders
        let mut cur_dir: & mut DirEntry = & mut ret;


        let mut path = PathBuf::from(file);
        //println!("Got file path {}", file);

        let mut components = path.components();
        let mut comp_vec: Vec<String> = Vec::new();

        for comp in components {
            //println!("{} ", comp.as_os_str().to_str().unwrap());
            comp_vec.push(String::from(comp.as_os_str().to_str().unwrap()));
        } 

        if comp_vec.len() > 1 {
            for i in 0..comp_vec.len() {
                if i == comp_vec.len()-1 {
                    // is file 
                    (*cur_dir).add_file(comp_vec[i].clone());
                    //println!("Added file {} to {}", comp_vec[i].clone(), cur_dir.get_name());
                }else{
                    let mut folder_name = String::from(comp_vec[i].clone());

                    if !cur_dir.contains_dir(String::from(comp_vec[i].clone())) {
                        let mut new_dir = DirEntry{cdir: folder_name, dirs: Vec::new(), files: Vec::new()};
                        let mut i : usize = (*cur_dir).add_dir(new_dir.clone());

                        cur_dir = & mut cur_dir.dirs[i];
                    }
                    if cur_dir.dirs.len() != 0 {
                        let mut i : usize = (*cur_dir).find_child_dir(String::from(comp_vec[i].clone()));
                        cur_dir = & mut cur_dir.dirs[i];
                    }

                }
                

            }
            cur_dir = & mut ret;
        }else{
            cur_dir = & mut ret;
            (*cur_dir).add_file(file.to_string());
        }
    }


    ret
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

