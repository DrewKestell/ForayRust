use crate::events::Event;

pub trait Observer {
    fn handle_event(&self, event: Event);
}