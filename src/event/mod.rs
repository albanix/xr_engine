use beryllium::events::Event;

pub trait EventHandler {
    fn handle(&self, event: Event);
}