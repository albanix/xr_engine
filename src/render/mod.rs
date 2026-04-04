use ogl33::{GL_COLOR_BUFFER_BIT, glClearColor, glClear};

pub struct Triangle;

impl Triangle {
    pub fn draw(&self) {
        unsafe {
            // думаю тут понятно
            glClearColor(0.2, 0.3, 0.3, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }
    }
}