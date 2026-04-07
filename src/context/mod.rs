use beryllium::{Sdl, error::SdlError, events::Event, init::InitFlags, video::{CreateWinArgs, GlWindow}};
use ogl33::load_gl_with;

pub struct Context {
    pub window: GlWindow,
    sdl: Sdl
}


impl Context {
    pub fn init(title: &str, h: i32, w: i32) -> Result<Self, SdlError> {
        let args = CreateWinArgs {
            title,
            width: w,
            height: h,
            ..Default::default()
        };

        let sdl = Sdl::init(InitFlags::EVERYTHING);
        let window = sdl.create_gl_window(args)?;
        sdl.set_gl_context_major_version(3)?;
        sdl.set_gl_profile(beryllium::video::GlProfile::Core)?;

        // Завантужуємо тут всі функції OpenGL
        unsafe {
            load_gl_with(|gl_f| window.get_proc_address(gl_f.cast()));
        }

        // Повертаємо екземпляр Context
        Ok( Self {
            window,
            sdl
        })
    }

    /// Створюємо вектор, та сунемо всі евенти туда ;3
    /// 
    pub fn event_polling(&self) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some((event, _timestamp)) = self.sdl.poll_events() {
            events.push(event);
        }

        events
    }

    pub fn swap(&self) {
        self.window.swap_window();
    }
}