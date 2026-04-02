use crate::engine::{draw::VERTEX, context::Context};
use ogl33::*;

pub struct Triangle {
    pub vertex: VERTEX,
    context: Context
}

impl Triangle {
    pub fn draw(&self) {
        unsafe {
            load_gl_with(|f_name| self.context.window.get_proc_address(f_name.cast()));

            // fuck you opengl           
        }
    }
}