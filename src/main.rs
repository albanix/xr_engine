mod render;
mod context;
mod event;
mod gl;
mod shader;

use beryllium::events::Event;

use crate::{
    context::Context,
    render::Triangle
};

struct App {
    triangle: Triangle
}

impl App {
    pub fn new() -> Self {
        Self {
            triangle: Triangle
        }
    }

    pub fn handle(&self, event: Event) {
        match event {
            Event::Quit => {
                std::process::exit(0);
            },
            _ => {}
        }
    }

    pub fn render(&self) {
        self.triangle.draw(
            [
                [-0.5, -0.5, 0.0],
                [0.5, -0.5, 0.0],
                [0.0, -0.5, 0.0]
            ]
        );
    }
}
fn main() {
    let context = Context::init("title", 400, 600).unwrap();
    let app = App::new();

    loop {
        for event in context.event_polling() {
            app.handle(event);
        }

        app.render();
        context.swap();
    }
}