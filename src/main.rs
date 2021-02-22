use fltk::*;
use fltk::{app::*, image::*, window::*};
use std::collections::HashMap;
use std::env;
use std::io::{self};

use std::path::PathBuf;
use std::{thread, time};
mod rzip;
use rzip::DirEntry;


#[derive(Copy, Clone)]
pub enum FileType {
    None,
    Zip,
    Rar,
    Lz,
    Gzip,
    Bzip,
    Tar,
}

#[derive(Copy, Clone)]
pub enum Message {
    FileCreateZip,
    FileOpen,
    ExtractAll,
    About,
    Exit,
}

#[derive(Clone)]
pub enum Item {
    Dir(DirEntry),
    File(String),
}

pub fn item_clicked_cb() {
    println!("item was clicked!");
}



pub fn start_gui() {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type(FrameType::BorderBox);

    let mut win = Window::new(500, 200, 600, 500, "rzip");
    let mut menubar = menu::SysMenuBar::new(0, 0, 600, 25, "");

    // x: 1, y: 27, h: 25, w: 25
    // yend: 25+27, xend: 1+25
    // ybeg: 27, xbeg: 1
    button::Button::new(1, 27, 25, 25, "🡠");

    let mut pathdisp = text::TextDisplay::new(25, 27, 600, 25, "");
    pathdisp.set_frame(FrameType::BorderBox);

    let (sender, receiver) = app::channel();
    //let  folder_ico = SharedImage::load("./icons/folder.png").unwrap();
    let file_ico = SharedImage::load("./icons/file.png").unwrap();

    // globals
    let mut path: String = "".to_string();
    let mut extract_path: String = "".to_string();
    let mut global_filepath: String = "".to_string();
    let mut current_filetype: FileType = FileType::None;

    let widths = &[250, 150];
    let mut b = browser::MultiBrowser::new(1, 55, 500, 400, "");

    b.add("File\tSize");
    b.set_column_char('\t');
    b.set_column_widths(widths);
    b.set_frame(FrameType::FlatBox);
    b.set_scrollbar_size(20);

    // map line to Item
    let mut itemsmap: HashMap<u32, Item> = HashMap::new();

    // directories traversed
    let mut nav_dirs: Vec<DirEntry> = Vec::new(); // first item is root dir

    menubar.add_emit(
        "&File/New Archive/Zip Archive\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        Message::FileCreateZip,
    );

    menubar.add_emit(
        "&File/Open \t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        sender,
        Message::FileOpen,
    );

    menubar.add_emit(
        "&File/Extract All \t",
        Shortcut::Ctrl | 'a',
        menu::MenuFlag::Normal,
        sender,
        Message::ExtractAll,
    );

    menubar.add_emit(
        "&File/Exit \t",
        Shortcut::Ctrl | 'e',
        menu::MenuFlag::Normal,
        sender,
        Message::Exit,
    );

    menubar.add_emit(
        "&Edit/Select all \t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        Message::Exit,
    );

    menubar.add_emit(
        "&Help/About rzip \t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        Message::About,
    );

    let mut lc: u32;

    let app_icon = image::PngImage::load("./icons/rzip.png").unwrap();
    win.set_icon(Some(app_icon));

    win.end();
    win.make_resizable(true);
    win.show();

    while app.wait() {
        if app::event_is_click() {
            // clicked on back btn
            if (app::event_x() < 26 && app::event_x() > 1)
                && (app::event_y() < 52 && app::event_y() > 27)
            {
                println!("Clicked on back button!");

                println!("Length of nav_dirs is {}", nav_dirs.len());

                if nav_dirs.len() != 1 {
                    let last = nav_dirs.len() - 2;
                    let prev_dir = nav_dirs.get_mut(last).unwrap();

                    itemsmap.clear();
                    let mut buf = pathdisp.buffer().unwrap();
                    let np = buf.line_text(1);

                    let split: Vec<&str> = np.as_str().split("\\").collect();
                    //let mut splitvec :  = split.collect();

                    let mut new_path: String = "".to_string();

                    for i in 0..split.len() - 2 {
                        new_path.push_str(split[i]);
                        new_path.push_str("\\");
                    }
                    path = new_path;
                    buf.set_text(path.as_str());

                    // clear lines
                    b.clear();
                    lc = 2;

                    b.add("File\tSize\t");
                    b.set_column_char('\t');
                    b.set_column_widths(widths);

                    for folder in prev_dir.dirs.clone() {
                        println!("Found a folder {} ", folder.get_name());

                        let fico = image::PngImage::load("./icons/folder.png").unwrap();

                        b.insert(lc, format!("{}\t-\t", folder.get_name().as_str()).as_str());
                        b.set_icon(lc, Some(fico));

                        itemsmap.insert(lc, Item::Dir(folder));
                        lc += 1;
                    }

                    for file in prev_dir.files.clone() {
                        println!("Found a file {} ", file.0);
                        //b.add(format!("{}\t\t\t\t\t", file).as_str());
                        b.insert(lc, format!("{}\t{}\t", file.0, file.1).as_str());
                        b.set_icon(lc, Some(file_ico.clone()));

                        itemsmap.insert(lc, Item::File(file.0));
                        lc += 1;
                    }

                    // pop last
                    nav_dirs.pop();
                }
            }

            let mut total_selected: u32 = 0;
            for i in 2..=b.size() {
                if b.selected(i) {
                    total_selected += 1;
                }
            }

            let mut selected: u32 = 0;
            if total_selected == 1 {
                for i in 2..=b.size() {
                    if b.selected(i) {
                        selected = i;
                        break;
                    }
                }
            }

            if selected != 0 {
                match itemsmap.clone().get(&selected).unwrap() {
                    Item::Dir(dirent) => {
                        // update navigated dirs
                        nav_dirs.push(dirent.clone());

                        path.push_str(dirent.get_name().as_str());
                        path.push_str("\\");

                        let mut new_pathbuf = text::TextBuffer::default();
                        new_pathbuf.set_text(path.as_str());
                        pathdisp.set_buffer(new_pathbuf);

                        itemsmap.clear();
                        // clear lines
                        b.clear();
                        lc = 2;

                        b.add("File\tSize\t");
                        b.set_column_char('\t');
                        b.set_column_widths(widths);

                        for folder in dirent.dirs.clone() {
                            println!("Found a folder {} ", folder.get_name());

                            let fico = image::PngImage::load("./icons/folder.png").unwrap();

                            b.insert(lc, format!("{}\t-\t", folder.get_name().as_str()).as_str());
                            b.set_icon(lc, Some(fico));

                            itemsmap.insert(lc, Item::Dir(folder));
                            lc += 1;
                        }

                        for file in dirent.files.clone() {
                            println!("Found a file {} ", file.0);
                            //b.add(format!("{}\t\t\t\t\t", file).as_str());
                            b.insert(lc, format!("{}\t{}\t", file.0, file.1).as_str());
                            b.set_icon(lc, Some(file_ico.clone()));

                            itemsmap.insert(lc, Item::File(file.0));
                            lc += 1;
                        }

                        println!("Clicked on folder {}", dirent.get_name());
                    }
                    Item::File(name) => {
                        println!("Clicked on file {}", name);
                    }
                }
            }
            //println!("Total selected: {}", total_selected);

            thread::sleep(time::Duration::from_millis(1000));
        }

        if let Some(msg) = receiver.recv() {
            match msg {
                Message::FileCreateZip => {
                    let mut default_file_path = "".to_owned();
                    let drivekey = "HOMEDRIVE";
                    match env::var(drivekey) {
                        Ok(v) => {
                            default_file_path.push_str(v.as_str());
                        }
                        Err(e) => println!("couldn't interpret {}: {}", drivekey, e),
                    }

                    let home_path = "HOMEPATH";
                    match env::var(home_path) {
                        Ok(val) => default_file_path.push_str(val.as_str()),
                        Err(e) => println!("couldn't interpret {}: {}", home_path, e),
                    }

                    default_file_path.push_str("\\Documents\\Archive.zip");
                    let zipfilepath = dialog::input(
                        500,
                        500,
                        "Enter the path to which you would like to extract to ",
                        default_file_path.as_str(),
                    )
                    .unwrap();

                    let mut fb = dialog::FileDialog::new(dialog::FileDialogType::BrowseMultiFile);
                    fb.show();
                    let filepaths = fb.filenames();

                    let zpf = zipfilepath.clone();
                    rzip::create_new_zip(zipfilepath, filepaths);

                    let (files, mut zipfilepath) = rzip::get_entries_from_file(PathBuf::from(zpf));
                    global_filepath = zipfilepath.clone();
                    zipfilepath.push_str("\\");

                    let split: Vec<&str> = zipfilepath.as_str().split('.').collect();
                    extract_path.push_str(split[0]);

                    let mut zipbuf = text::TextBuffer::default();
                    path = zipfilepath.as_str().to_string();
                    zipbuf.set_text(path.as_str());

                    nav_dirs.push(files.clone());

                    pathdisp.set_buffer(zipbuf);

                    // clear lines
                    b.clear();
                    lc = 2;

                    b.add("File\tSize\t");
                    b.set_column_char('\t');
                    b.set_column_widths(widths);

                    for folder in files.dirs {
                        //println!("Found a folder {} ", folder.get_name());

                        //b.add(format!("{}\t\t\t\t\t", folder.get_name()).as_str());
                        let fico = image::PngImage::load("./icons/folder.png").unwrap();
                        b.insert(lc, format!("{}\t-\t", folder.get_name().as_str()).as_str());
                        b.set_icon(lc, Some(fico));

                        itemsmap.insert(lc, Item::Dir(folder));
                        lc += 1;
                    }

                    for file in files.files {
                        //println!("Found a file {} ", file.0);

                        b.insert(lc, format!("{}\t{}\t", file.0, file.1).as_str());
                        b.set_icon(lc, Some(file_ico.clone()));

                        itemsmap.insert(lc, Item::File(file.0));
                        lc += 1;
                    }
                }
                Message::FileOpen => {
                    let (files, mut filepath) = rzip::get_entries();
                    global_filepath = filepath.clone();
                    let pbuf = PathBuf::from(filepath.clone());
                    let ext = pbuf.as_path().extension().unwrap().to_str().unwrap();

                    match ext {
                        "zip" => {
                            current_filetype = FileType::Zip;
                        }
                        "bzip2" => {
                            current_filetype = FileType::Bzip;
                        }
                        "rar" => {
                            current_filetype = FileType::Rar;
                        }
                        "7z" => {
                            current_filetype = FileType::Lz;
                        }
                        "gz" => {
                            current_filetype = FileType::Gzip;
                        }
                        "tar" => {
                            current_filetype = FileType::Tar;
                        }
                        _ => {
                            current_filetype = FileType::None;
                        }
                    }

                    filepath.push_str("\\");

                    let split: Vec<&str> = filepath.as_str().split('.').collect();
                    extract_path.push_str(split[0]);

                    let mut zipbuf = text::TextBuffer::default();
                    path = filepath.as_str().to_string();
                    zipbuf.set_text(path.as_str());

                    nav_dirs.push(files.clone());

                    pathdisp.set_buffer(zipbuf);

                    // clear lines
                    b.clear();
                    lc = 2;

                    b.add("File\tSize\t");
                    b.set_column_char('\t');
                    b.set_column_widths(widths);

                    for folder in files.dirs {
                        //println!("Found a folder {} ", folder.get_name());

                        //b.add(format!("{}\t\t\t\t\t", folder.get_name()).as_str());
                        let fico =
                            image::PngImage::load("./icons/folder.png").expect("Image not loaded");
                        b.insert(lc, format!("{}\t-\t", folder.get_name().as_str()).as_str());
                        b.set_icon(lc, Some(fico));

                        itemsmap.insert(lc, Item::Dir(folder));
                        lc += 1;
                    }

                    for file in files.files {
                        //println!("Found a file {} ", file.0);

                        b.insert(lc, format!("{}\t{}\t", file.0, file.1).as_str());
                        b.set_icon(lc, Some(file_ico.clone()));

                        itemsmap.insert(lc, Item::File(file.0));
                        lc += 1;
                    }
                }
                Message::ExtractAll => match current_filetype {
                    FileType::Zip => {
                        println!("Trying to extract a zip file");
                        let default_extract_path = extract_path.clone();
                        let input_path = dialog::input(
                            500,
                            500,
                            "Enter the path to which you would like to extract to ",
                            default_extract_path.as_str(),
                        )
                        .unwrap();

                        rzip::unzip(global_filepath.clone(), PathBuf::from(input_path));

                        dialog::alert(500, 500, "Extraction successful");
                        dialog::beep(dialog::BeepType::Default);
                    }
                    FileType::Rar => {
                        let default_extract_path = extract_path.clone();
                        let input_path = dialog::input(
                            500,
                            500,
                            "Enter the path to which you would like to extract to ",
                            default_extract_path.as_str(),
                        )
                        .unwrap();

                        rzip::unrar(global_filepath.clone(), PathBuf::from(input_path));

                        dialog::alert(500, 500, "Extraction successful");
                        dialog::beep(dialog::BeepType::Default);

                        println!("Trying to extract a rar file");
                    }
                    FileType::Bzip => {
                        println!("Trying to extract a bzip file");
                    }
                    FileType::Lz => {
                        println!("Trying to extract a 7z file");
                    }
                    FileType::Gzip => {
                        println!("Trying to extract a gzip file");
                    }
                    FileType::Tar => {
                        println!("Trying to extract a tar file");
                    }
                    FileType::None => {
                        dialog::alert(500, 500, "You must have a supported file open");
                    }
                },
                Message::Exit => {
                    app.quit();
                }
                Message::About => {
                    fltk::dialog::message(800, 500, "Made by stoozy (c) 2021");
                }
            }
        }
    }
}

pub fn get_char() -> char {
    let mut buf = String::new();
    let stdin = io::stdin();

    match stdin.read_line(&mut buf) {
        Err(e) => println!("Error occured: {}", e),
        Ok(n) => {
            if n != 0 {
                return buf.as_bytes()[0] as char;
            }
        }
    }
    if buf.len() != 0 {
        return buf.as_bytes()[0] as char;
    } else {
        return '/';
    }
}

pub fn get_string_input() -> String {
    let mut buf = String::new();
    let stdin = io::stdin();

    match stdin.read_line(&mut buf) {
        Err(e) => println!("Error occured: {}", e),
        Ok(_) => {
            buf.pop();
            buf.pop();
            return buf;
        }
    }
    return buf;
}

pub fn vec_contains(vec: Vec<String>, find: String) -> bool {
    for item in vec {
        if item == find {
            return true;
        }
    }
    return false;
}

pub fn main() {
    let mut argc = 0;
    let mut args: Vec<String> = vec![];

    let cwd: String = env::current_dir().unwrap().display().to_string();

    for arg in env::args() {
        args.push(arg);
        argc += 1;
    }

    if argc == 0 {
        start_gui();
    } else {
        if !vec_contains(args, "--nogui".to_string()) {
            start_gui();
        } else {
            println!("Welcome to rzip!\n\t(a) Create new zip file\n\t(b) Unzip a file\n\t(c) exit\nPlease select an option:");
            let inp = get_char();
            if inp == 'a' {
                println!("Enter the name of your file: ");
                let filename: String = get_string_input();
                let mut filepaths: Vec<String> = Vec::new();
                let mut files: Vec<String> = Vec::new();

                let mut done_getting_files: bool = false;
                while !done_getting_files {
                    println!("Please enter next file (type 'done' to stop): ");
                    let file = get_string_input();

                    if file == "done".to_string() {
                        done_getting_files = true;
                    } else if !file.is_empty() {
                        let mut path = cwd.clone();
                        path.push_str("\\");
                        path.push_str(file.as_str());

                        filepaths.push(path);
                        files.push(file);
                    }
                }

                rzip::zip_files(filename, filepaths, files);
            } else if inp == 'b' {
                println!("Please enter the path of the zip file: ");
                let zippath = PathBuf::from(get_string_input());
                let extract_dir = zippath.file_stem().unwrap();
                println!("File will be extracted to {:?}", extract_dir);

                rzip::unzip(
                    zippath.as_path().display().to_string(),
                    PathBuf::from(extract_dir),
                );
            }
        }
    }
}
