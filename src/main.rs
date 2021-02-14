use fltk::{app::*, button::*, frame::*, window::*};
use fltk::*;

mod callbacks{
    pub fn open_cb(){
    }

}

pub fn main(){
    let app = App::default();
    app::background(255,255,255);

    let mut win = Window::new(500, 200, 400, 300, "rzip");
    let mut frame = Frame::new(0,0,400, 200, "");

    let mut menubar = menu::SysMenuBar::new(0, 0, 600, 25, "Test");

    let test  = "pogg";
    let fmi = menu::MenuItem::new(&[test]);

    menubar.add(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        callbacks::open_cb,
    );
    
    win.end();
    win.show();
    app.run().unwrap();
}