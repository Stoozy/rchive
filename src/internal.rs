use unrar::error::UnrarError;

use fltk::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct DirEntry {
    cdir: String,
    pub dirs: Vec<DirEntry>,
    pub files: Vec<(String, u64)>,
}

impl DirEntry {
    pub fn get_name(&self) -> String {
        return self.cdir.clone();
    }
    pub fn get_dirs(&self) -> Vec<DirEntry> {
        return self.dirs.clone();
    }
    pub fn get_files(&self) -> Vec<(String, u64)> {
        return self.files.clone();
    }

    pub fn contains_dir(&self, find: String) -> bool {
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

    pub fn add_dir(&mut self, new_dir: DirEntry) -> usize {
        let mut idx: usize = 0;
        let name = new_dir.get_name();
        self.dirs.push(new_dir);
        for i in 0..self.dirs.len() {
            if self.dirs[i].get_name() == name {
                idx = i;
            }
        }
        return idx;
    }

    pub fn add_file(&mut self, file: String, size: u64) -> () {
        self.files.push((file, size));
    }
}

pub fn get_file_from_path(path: String) -> File {
    let file = File::open(path).unwrap();
    return file;
}

pub fn create_new_zip(file: String, filepaths: Vec<PathBuf>) {
    let zipfile = File::create(file).unwrap();
    let mut zip = zip::ZipWriter::new(zipfile);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for i in 0..filepaths.len() {
        let cfp = &filepaths[i];
        //let comp_list : Vec<&str> = cfp.components().collect();
        let cf: String = cfp
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        println!("Got file {}", cf.as_str());

        match fs::read(cfp) {
            Ok(val) => {
                println!("Writing {} to zip file", cf.as_str());
                zip.start_file(cf, options).unwrap();
                zip.write(&val).unwrap();
            }
            Err(e) => {
                println!("Error occured: {}", e);
                //println!("Path was: {}", cfp.into_os_string().into_string().unwrap());
            }
        }
    }

    zip.finish().unwrap();
}

pub fn zip_files(mut file: String, filepaths: Vec<String>, files: Vec<String>) -> () {
    // add file extension before creating
    file.push_str(".zip");
    println!("Zip file is {}", file);

    let zipfile = File::create(file).unwrap();
    let mut zip = zip::ZipWriter::new(zipfile);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for i in 0..filepaths.len() {
        let cfp = &filepaths[i];
        let cf = &files[i];
        match fs::read(cfp) {
            Ok(val) => {
                println!("Writing {} to zip file", cf);
                zip.start_file(cf, options).unwrap();
                zip.write(&val).unwrap();
            }
            Err(e) => {
                println!("Error occured: {}", e);
                println!("Path was: {}", cfp);
            }
        }
    }

    zip.finish().unwrap();

    info!("Successfully created zip file!");
}

pub fn unzip(filename: String, dir_path: PathBuf) {
    let zipfile = get_file_from_path(filename);
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    match zip.extract(dir_path){
        Ok(_) => debug!("Extraction sucessful"),
        Err(e) => error!("Error occured: {}", e),
    };
}

pub fn unrar(filename: String, dir_path: PathBuf) {
    match unrar::archive::Archive::new(filename)
        .extract_to(dir_path.display().to_string())
        .unwrap()
        .process()
        {
            Ok(_) =>  info!("Extracted rar archive"),
            Err(e) => error!("{}", e),
        }
}

pub fn get_entries() -> Option<(DirEntry, String)> {
    let mut browser = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
    browser.set_filter("");
    browser.show();

    let path: std::path::PathBuf = browser.filename();
    info!("Chosen file is: {:?}", path.to_str());

    let ext = path.extension().unwrap().to_str().unwrap();

    match ext {
        "zip" => {
            info!("Opened a zip file");
            let filepath: String = String::from(path.to_str().unwrap());

            let file = File::open(&path).unwrap();
            let zipfile = zip::ZipArchive::new(file).unwrap();

            return Some(list_zip(zipfile, filepath));
        }
        "rar" => {
            let filepath: String = String::from(path.to_str().unwrap());
            let rarfile = unrar::archive::Archive::new(path.to_str().unwrap().to_string());
            match rarfile.list_split() {
                Ok(archive) => return Some(list_rar(archive, filepath)),

                // If the error's data field holds an OpenArchive, an error occurred while opening,
                // the archive is partly broken (e.g. broken header), but is still readable from.
                // In this example, we are still going to use the archive and list its contents.
                Err(error @ UnrarError { data: Some(_), .. }) => {
                    //writeln!(&mut stderr, "Error: {}, continuing.", error).unwrap();
                    list_rar(error.data.unwrap(), filepath);
                }
                // Irrecoverable failure, do nothing.
                Err(e) => {
                    error!("Error occured: {}", e);
                    //writeln!(&mut stderr, "Error: {}", e).unwrap();
                }
            }

            info!("Opened a rar file");
        }
        "bzip2" => {
            info!("Opened a bzip file")
        }
        "bz2" => {
            info!("Opened a bzip file")
        }
        _ => {
            info!("Opened  a random file");
            dialog::alert(550, 300, "Cannot open this filetype");
            return None;
        }
    };

    return None; 
}

pub fn get_entries_from_file(path: PathBuf) -> (DirEntry, String) {
    let mut ret: DirEntry = DirEntry {
        cdir: String::from("/"),
        dirs: Vec::new(),
        files: Vec::new(),
    };
    // TODO: check file ext for compatibility (.zip only for now)
    println!("Chosen file is: {:?}", path.to_str());

    let filepath: String = String::from(path.to_str().unwrap());

    let file = File::open(&path).unwrap();
    let mut zipfile = zip::ZipArchive::new(file).unwrap();

    //let mut files = zipfile.file_names();

    // add dir if not found
    // continue until parent dir of file is found
    // add file to parent dirs files
    // repeat for every file
    for i in 0..zipfile.len() {
        let file = zipfile.by_index(i).unwrap();

        // has folders
        let mut cur_dir: &mut DirEntry = &mut ret;

        let path = PathBuf::from(file.name());
        //println!("Got file path {}", file);

        let components = path.components();
        let mut comp_vec: Vec<String> = Vec::new();

        for comp in components {
            //println!("{} ", comp.as_os_str().to_str().unwrap());
            comp_vec.push(String::from(comp.as_os_str().to_str().unwrap()));
        }

        if comp_vec.len() > 1 {
            for i in 0..comp_vec.len() {
                if i == comp_vec.len() - 1 {
                    // is file
                    (*cur_dir).add_file(comp_vec[i].clone(), file.size());
                    //println!("Added file {} to {}", comp_vec[i].clone(), cur_dir.get_name());
                } else {
                    let folder_name = String::from(comp_vec[i].clone());

                    if !cur_dir.contains_dir(String::from(comp_vec[i].clone())) {
                        let new_dir = DirEntry {
                            cdir: folder_name,
                            dirs: Vec::new(),
                            files: Vec::new(),
                        };
                        let i: usize = (*cur_dir).add_dir(new_dir.clone());

                        cur_dir = &mut cur_dir.dirs[i];
                    }
                    if cur_dir.dirs.len() != 0 {
                        let i: usize = (*cur_dir).find_child_dir(String::from(comp_vec[i].clone()));
                        cur_dir = &mut cur_dir.dirs[i];
                    }
                }
            }
            //cur_dir = &mut ret;
        } else {
            cur_dir = &mut ret;
            (*cur_dir).add_file(file.name().to_string(), file.size());
        }
    }

    (ret, filepath)
}

pub fn list_rar(archive: unrar::archive::OpenArchive, filepath: String) -> (DirEntry, String) {
    let mut ret: DirEntry = DirEntry {
        cdir: String::from("/"),
        dirs: Vec::new(),
        files: Vec::new(),
    };
    let mut stderr = std::io::stderr();

    // need two entries
    // one for adding all folders
    // one for adding all files
    for entry in archive {
        match entry {
            Ok(file) => {
                let mut cur_dir: &mut DirEntry = &mut ret;
                let pathb = PathBuf::from(file.to_string());
                let components = pathb.components();
                let mut comp_vec = Vec::new();
                for comp in components {
                    //println!("{} ", comp.as_os_str().to_str().unwrap());
                    comp_vec.push(String::from(comp.as_os_str().to_str().unwrap()));
                }

                if file.is_file() {
                    if comp_vec.len() > 1 {
                        for i in 0..comp_vec.len() {
                            if i == comp_vec.len() - 1 {
                                // is file
                                (*cur_dir).add_file(comp_vec[i].clone(), file.unpacked_size as u64);
                                //println!("Added file {} to {}", comp_vec[i].clone(), cur_dir.get_name());
                            } else {
                                let folder_name = String::from(comp_vec[i].clone());

                                if !cur_dir.contains_dir(String::from(comp_vec[i].clone())) {
                                    let new_dir = DirEntry {
                                        cdir: folder_name,
                                        dirs: Vec::new(),
                                        files: Vec::new(),
                                    };
                                    let i: usize = (*cur_dir).add_dir(new_dir.clone());

                                    cur_dir = &mut cur_dir.dirs[i];
                                }
                                if cur_dir.dirs.len() != 0 {
                                    let i: usize = (*cur_dir)
                                        .find_child_dir(String::from(comp_vec[i].clone()));
                                    cur_dir = &mut cur_dir.dirs[i];
                                }
                            }
                        }
                        //cur_dir = &mut ret;
                    } else {
                        cur_dir = &mut ret;
                        (*cur_dir).add_file(file.filename, file.unpacked_size as u64);
                    }
                }
                //println!("File : {} Is File: {} Is Dir: {}", e.filename, e.is_file(), e.is_directory());
            }
            Err(err) => writeln!(&mut stderr, "Error: {}", err).unwrap(),
        }
    }

    (ret, filepath)
}

pub fn list_zip(
    mut zipfile: zip::ZipArchive<std::fs::File>,
    filepath: String,
) -> (DirEntry, String) {
    let mut ret: DirEntry = DirEntry {
        cdir: String::from("/"),
        dirs: Vec::new(),
        files: Vec::new(),
    };
    // add dir if not found
    // continue until parent dir of file is found
    // add file to parent dirs files
    // repeat for every file
    for i in 0..zipfile.len() {
        let file = zipfile.by_index(i).unwrap();

        // has folders
        let mut cur_dir: &mut DirEntry = &mut ret;

        let path = PathBuf::from(file.name());
        //println!("Got file path {}", file);

        let components = path.components();
        let mut comp_vec: Vec<String> = Vec::new();

        for comp in components {
            //println!("{} ", comp.as_os_str().to_str().unwrap());
            comp_vec.push(String::from(comp.as_os_str().to_str().unwrap()));
        }

        if comp_vec.len() > 1 {
            for i in 0..comp_vec.len() {
                if i == comp_vec.len() - 1 {
                    // is file
                    (*cur_dir).add_file(comp_vec[i].clone(), file.size());
                    //println!("Added file {} to {}", comp_vec[i].clone(), cur_dir.get_name());
                } else {
                    let folder_name = String::from(comp_vec[i].clone());

                    if !cur_dir.contains_dir(String::from(comp_vec[i].clone())) {
                        let new_dir = DirEntry {
                            cdir: folder_name,
                            dirs: Vec::new(),
                            files: Vec::new(),
                        };
                        let i: usize = (*cur_dir).add_dir(new_dir.clone());

                        cur_dir = &mut cur_dir.dirs[i];
                    }
                    if cur_dir.dirs.len() != 0 {
                        let i: usize = (*cur_dir).find_child_dir(String::from(comp_vec[i].clone()));
                        cur_dir = &mut cur_dir.dirs[i];
                    }
                }
            }
            //cur_dir = &mut ret;
        } else {
            cur_dir = &mut ret;
            (*cur_dir).add_file(file.name().to_string(), file.size());
        }
    }

    return (ret, filepath);
}
