use fltk::{widget::*,browser::*, app::*, button::*, image::*, frame::*, window::*};
use fltk::*;
use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::{thread, time};
mod rzip;
use std::ops::{Deref, DerefMut};
use rzip::DirEntry;





#[derive(Copy, Clone)]
pub enum Message{
    FileCreate,
    FileOpen,
    ExtractAll,
    About,
    Exit,
}

#[derive(Clone)]
pub enum Item {
    Dir(DirEntry),
    File(String)
}

pub fn item_clicked_cb(){
    println!("item was clicked!");
}

pub fn start_gui(){

    let app = App::default();
    app::background(255, 255, 255);
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type(FrameType::BorderBox);

    let mut win = Window::new(500, 200, 600, 500, "rzip");
    let mut menubar = menu::SysMenuBar::new(0, 0, 600, 25, "");
    let frame = Frame::new(0,0,400, 200, "");

    //let mut list = ListWidget::new(0, 30, 500, 500, "");

    let mut nav_back_btn = button::Button::new(1, 27, 25, 25, "🡠");
    let mut pathbuf = text::TextBuffer::default();
    let mut pathdisp = text::TextDisplay::new(25, 27, 550, 25, "") ;
    pathbuf.set_text("");
    pathdisp.set_buffer(Some(pathbuf));
    pathdisp.set_frame(FrameType::BorderBox);

    let (sender, receiver) = app::channel();
    let mut folder_ico = SharedImage::load("./icons/folder.png").unwrap();
    let mut file_ico = SharedImage::load("./icons/file.png").unwrap();


    //let mut tab = table::Table::new(0,50,600, 500, "");
    //tab.set_table_frame(FrameType::BorderBox);
    //tab.set_col_header(true);
    //tab.set_col_header_height(20);
    //tab.set_cols(5);
    //tab.set_rows(5);
    //tab.set_col_width_all(200);
    //tab.set_top_row(20);

    let widths = &[250, 150, 150, 150, 150];
    let mut b = browser::MultiBrowser::new(1,55,600, 500, "");

    b.add("File\tSize\tLast Changed\tCreated\tAccessed\t");
    b.set_column_char('\t');
    b.set_column_widths(widths);


    // map line to Item
    let mut itemsmap : HashMap<u32, Item> = HashMap::new();

    menubar.add_emit(
        "&File/New Archive\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        sender,
        Message::FileCreate
    );

    menubar.add_emit(
        "&File/Open \t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        sender,
        Message::FileOpen
    );
    
    menubar.add_emit(
        "&File/Extract All \t",
        Shortcut::Ctrl | 'a',
        menu::MenuFlag::Normal,
        sender,
        Message::ExtractAll
    );


    menubar.add_emit(
        "&File/Exit \t",
        Shortcut::Ctrl | 'e',
        menu::MenuFlag::Normal,
        sender,
        Message::Exit
    );

    menubar.add_emit(
        "&Edit/Select all \t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        Message::Exit
    );

    menubar.add_emit(
        "&Help/About rzip \t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        Message::About
    );


    
    let mut lc : u32 = 2;
   
    let app_icon = image::PngImage::load("./icons/rzip.png").unwrap();
    win.set_icon(Some(app_icon));


    win.end();
    win.make_resizable(true);
    win.show();

    while app.wait(){

        if app::event_is_click() {
            let mut total_selected : u32 = 0;
            for i in 2..=b.size(){
                if b.selected(i){
                    total_selected += 1;
                }
            }


            let mut selected : u32 = 0;
            if total_selected == 1 {
                for i in 2..=b.size() {
                    if b.selected(i){
                        selected = i;
                        break;
                    }
                }
            } 
            if selected != 0 {
            match itemsmap.clone().get(&selected).unwrap() {
                Item::Dir(dirent) => {
                    itemsmap.clear();
                    // clear lines
                    b.clear();
                    lc = 2;

                    b.add("File\tSize\tLast Changed\tCreated\tAccessed\t");
                    b.set_column_char('\t');
                    b.set_column_widths(widths);


                    for mut folder in dirent.dirs.clone() {
                        println!("Found a folder {} ", folder.get_name());

                        //b.add(format!("{}\t\t\t\t\t", folder.get_name()).as_str());
                        let fico = image::PngImage::load("./icons/folder.png").unwrap();

                        b.insert(lc, format!("{}\t\t\t\t\t", folder.get_name().as_str()).as_str());
                        b.set_icon(lc, Some(fico));
                        
                        itemsmap.insert(lc, Item::Dir(folder));
                        lc +=1;
                    }

                    for mut file in dirent.files.clone() {
                        println!("Found a file {} ", file);
                        //b.add(format!("{}\t\t\t\t\t", file).as_str());
                        b.insert(lc, format!("{}\t\t\t\t\t", file).as_str());
                        b.set_icon(lc, Some(file_ico.clone()));

                        itemsmap.insert(lc, Item::File(file));
                        lc+=1;
                    }
                   

                    println!("Clicked on folder {}", dirent.get_name());
                },
                Item::File(name) => {
                    println!("Clicked on file {}", name);
                }
            }


            }
            //println!("Total selected: {}", total_selected);

            thread::sleep(time::Duration::from_millis(1000));
        }

        if let Some(msg) =  receiver.recv(){
            match msg{
                Message::FileCreate => {
                    rzip::file_new_handler();
                },
                Message::FileOpen => {
                    let (mut files, mut zipfilepath) = rzip::get_entries();
                    //pathbuf.set_text(zipfilepath.as_str());
                    let mut zipbuf = text::TextBuffer::default();
                    zipbuf.set_text(zipfilepath.as_str());

                    pathdisp.set_buffer(zipbuf);
                    // clear lines
                    b.clear();
                    lc = 2;

                    b.add("File\tSize\tLast Changed\tCreated\tAccessed\t");
                    b.set_column_char('\t');
                    b.set_column_widths(widths);


                    for mut folder in files.dirs {
                        println!("Found a folder {} ", folder.get_name());

                        //b.add(format!("{}\t\t\t\t\t", folder.get_name()).as_str());
                        let fico = image::PngImage::load("./icons/folder.png").unwrap();
                        b.insert(lc, format!("{}\t\t\t\t\t", folder.get_name().as_str()).as_str());
                        b.set_icon(lc, Some(fico));

                        itemsmap.insert(lc, Item::Dir(folder));
                        lc +=1;
                    }

                    for mut file in files.files {
                        println!("Found a file {} ", file);
                        //b.add(format!("{}\t\t\t\t\t", file).as_str());
                        b.insert(lc, format!("{}\t\t\t\t\t", file).as_str());
                        b.set_icon(lc, Some(file_ico.clone()));

                        itemsmap.insert(lc, Item::File(file));
                        lc+=1;
                    }
                    

                },
                Message::ExtractAll => {
                    println!("Extract all clicked!");
                },
                Message::Exit =>{
                    app.quit();
                },
                Message::About => {
                    fltk::dialog::message(800, 500, "Made by stoozy (c) 2021");
                }

            }

        }
    }

}

pub fn get_char() -> char {
    let mut buf = String::new();
    let mut stdin = io::stdin();

    match stdin.read_line(&mut buf) {
        Err(e) => println!("Error occured: {}", e),
        Ok(n) => {
            if n!=0{
                return buf.as_bytes()[0] as char;
            }
        },
    }
    if buf.len() != 0{
        return buf.as_bytes()[0] as char;
    }else{
        return '/';
    }
}

pub fn get_string_input() -> String{
    let mut buf = String::new();
    let mut stdin = io::stdin();

    match stdin.read_line(&mut buf) {
        Err(e) => println!("Error occured: {}", e),
        Ok(_) => {
            buf.pop();
            buf.pop();
            return buf;
        },
    }
    return buf;
}

pub fn vec_contains(mut vec : Vec<String>, find : String) -> bool {
    for item in vec {
        if item == find {
            return true;
        }
    }
    return false;
}

pub fn main(){
    let mut argc = 0;
    let mut args:Vec<String> = vec![];

    let mut cwd : String = env::current_dir().unwrap().display().to_string();
    
    for arg in env::args(){
        args.push(arg);
        argc+=1;
    }

    if argc==0 {
        start_gui();
    } else{
        if !vec_contains(args, "--nogui".to_string()) {
            start_gui();
        }else{
            println!("Welcome to rzip!\n\t(a) Create new zip file\n\t(b) Unzip a file\n\t(c) exit\nPlease select an option:");
            let mut buf = String::new();
            let mut stdin = io::stdin();
            let inp = get_char();
            if inp == 'a' {
                println!("Enter the name of your file: ");
                let filename :String = get_string_input();
                let mut filepaths: Vec<String> = Vec::new();
                let mut files: Vec<String>  = Vec::new();

                let mut done_getting_files : bool = false; 
                while !done_getting_files {
                    println!("Please enter next file (type 'done' to stop): ");
                    let mut file = get_string_input();                           

                    if file == "done".to_string() {
                        done_getting_files = true;
                    }else if !file.is_empty() {
                        let mut path = cwd.clone();
                        path.push_str("\\");
                        path.push_str(file.as_str());
                        
                        filepaths.push(path);
                        files.push(file);
                    }
                }

                rzip::zip_files(filename, filepaths, files);

            }else if inp == 'b' {                    
                println!("Please enter the path of the zip file: ");
                let zippath = PathBuf::from(get_string_input());
                let extract_dir = zippath.file_stem().unwrap();
                println!("File will be extracted to {:?}", extract_dir);

                rzip::unzip(zippath.as_path().display().to_string() , PathBuf::from(extract_dir));
            }

        }

    }

}

