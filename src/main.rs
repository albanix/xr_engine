mod render;
mod context;
mod event;

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
        self.triangle.draw();
    }
}
fn main() {
    let context = Context::init("title", 400, 600).unwrap();
    let mut app = App::new();

    loop {
        for event in context.event_polling() {
            app.handle(event);
        }

        app.render();
        context.swap();
    }
}