mod platform;
mod error;
mod core;

use crate::{
    platform::Platform
};
fn main() {
    let sdl = Platform::new().unwrap();
    let win = sdl.create_window().unwrap();
    sdl.sdl_poll(&win);
}