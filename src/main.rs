use beryllium::*;

use crate::engine::context::Context;

mod engine;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    // ==============================================
    let win = Context::new(
        "title",
        600,
        400,
        true,
        false,
        false,
        sdl
    );


    win.run();
}
