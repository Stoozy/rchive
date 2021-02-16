use fltk::{browser::*, app::*, button::*, image::*, frame::*, window::*};
use fltk::*;
use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
mod events;
mod rzip;



#[derive(Copy, Clone)]
pub enum Message{
    FileCreate,
    FileOpen,
    Exit,
}

pub fn start_gui(){

    let app = App::default();
    app::background(255, 255, 255);
    app::set_scheme(app::Scheme::Base);

    let mut win = Window::new(500, 200, 600, 600, "rzip");
    let frame = Frame::new(0,0,400, 200, "");


    let mut menubar = menu::SysMenuBar::new(0, 0, 600, 25, "");

    let (sender, receiver) = app::channel();
    let mut folder_ico = SharedImage::load("./icons/folder.png").unwrap();

    let mut table = table::Table::new(5,20,500,500 ,"");

    //let widths = &[100, 150, 150, 150, 150];
    //let mut b = browser::MultiBrowser::new(1,25,580, 500, "");

    //b.add("File\tSize\tLast Changed\tCreated\tAccessed\t");
    //b.set_column_char('\t');
    //b.set_column_widths(widths);
    //b.add("One\t20\t02/01/2021\tYesterday\tToday\t");
    //b.set_icon(2, Some(folder_ico));



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
        "&File/Exit \t",
        Shortcut::Ctrl | 'e',
        menu::MenuFlag::Normal,
        sender,
        Message::Exit
    );

   
    win.end();
    win.make_resizable(true);
    win.show();
    while app.wait(){
        if let Some(msg) =  receiver.recv(){
            match msg{
                Message::FileCreate =>{
                    events::file_new_handler();
                },
                Message::FileOpen => {

                    let files = events::get_entries();
                    for i in &files{
                        println!("{}", i);
                    }

                },
                Message::Exit =>{
                    app.quit();
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
        if(!vec_contains(args, "--nogui".to_string())){
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

                 
            }

        }

    }
}

