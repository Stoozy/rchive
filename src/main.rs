use fltk::{browser::*, app::*, button::*, image::*, frame::*, window::*};
use fltk::*;
mod events;

#[derive(Copy, Clone)]
pub enum Message{
    FileCreate,
    FileOpen,
    Exit,
}

pub fn main(){

    let app = App::default();
    app::background(255, 255, 255);

    let mut win = Window::new(500, 200, 600, 600, "rzip");
    let frame = Frame::new(0,0,400, 200, "");


    let mut menubar = menu::SysMenuBar::new(0, 0, 600, 25, "");

    let (sender, receiver) = app::channel();
    let mut folder_ico = SharedImage::load("./icons/folder.png").unwrap();

    let widths = &[100, 150, 150, 150, 150];
    let mut b = browser::MultiBrowser::new(1,25,580, 500, "");
    b.add("File\tSize\tLast Changed\tCreated\tAccessed\t");
    b.set_column_char('\t');
    b.set_column_widths(widths);
    b.add("One\t20\t02/01/2021\tYesterday\tToday\t");
    b.set_icon(2, Some(folder_ico));



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