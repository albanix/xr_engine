use beryllium::{Sdl, error::SdlError, events::Event, init::InitFlags, video::{CreateWinArgs, GlWindow}};

pub trait EventHandler {
    fn handle(&self, event: Event);
}

pub struct Context<T: EventHandler> {
    window: GlWindow,
    sdl: Sdl,
    handler: T
}


impl<T: EventHandler> Context<T> {
    pub fn init(title: &str, h: i32, w: i32, handler: T) -> Result<Self, SdlError> {
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

        Ok( Self {
            window,
            sdl,
            handler
        })
    }

    pub fn event_polling(&self) {
        'main_loop: loop {
            while let Some((event, _timestamp)) = self.sdl.poll_events() {
                match event {
                    Event::Quit => break 'main_loop,
                    _ => self.handler.handle(event)
                }
            }

            self.window.swap_window();
        }
    }
}