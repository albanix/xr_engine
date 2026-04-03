mod render;
mod context;

use beryllium::events::Event;

use crate::context::{
    EventHandler,
    Context
};

struct App;

impl EventHandler for App {
    fn handle(&self, _event: beryllium::events::Event) {
        
    }
}
fn main() {
    let h = App;
    let context = Context::init("title", 400, 600, h).unwrap();
    context.event_polling();
}