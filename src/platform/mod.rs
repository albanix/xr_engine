use beryllium::{
    Sdl, events::Event, init::InitFlags, video::{CreateWinArgs, GlWindow}
};
use ogl33::*;

use super::error::PlatformError;
pub struct Platform {
    sdl: Sdl    
}

pub struct Window {
    window: GlWindow
}

impl Platform {
    pub fn new() -> Result<Self, PlatformError> {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        sdl.set_gl_context_major_version(3)
            .map_err(|_| PlatformError::Gl)?;

        Ok(Self {
            sdl
        })
    }

    pub fn create_window(&self) -> Result<Window, PlatformError> {
        let window = self.sdl.create_gl_window(CreateWinArgs {
            title: "Engine Title",
            width: 600,
            height: 600,
            ..Default::default()
        }).map_err(|_| PlatformError::CreateWindowError)?;

        Ok(Window { window })
    }

    pub fn sdl_poll(&self, win: &Window) {
        'main_loop: loop {
            while let Some((event, _timstamp)) = self.sdl.poll_events() {
                match event {
                    Event::Quit => break 'main_loop,
                    _ => {}
                }
            }
        }
        
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
        }
        win.swap_buff();
    } 
}

impl Window {
    pub fn swap_buff(&self) {
        self.window.swap_window();
    }
}