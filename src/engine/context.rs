use beryllium::{Sdl, events, video::{CreateWinArgs, GlWindow}};

pub struct Context {
    pub window: GlWindow,
    pub sdl: Sdl
}

impl Context {
    pub fn new(
        title: &str,
        width: i32,
        height: i32,
        allow_high_dpi: bool,
        borderless: bool,
        resizable: bool,
        sdl: Sdl
    ) -> Self {
        let args = CreateWinArgs {
            title,
            width,
            height,
            allow_high_dpi,
            borderless,
            resizable
        };
        let window = sdl.create_gl_window(args)
            .expect("bk=lyat");
        Self {
            window,
            sdl
        }
    }

    pub fn run(&self) {
        'main_loop: loop {
            while let Some((event, _timestamp)) = self.sdl.poll_events() {
                match event {
                    events::Event::Quit => break 'main_loop,
                    _ => {}
                }
            }
        }
    }
}