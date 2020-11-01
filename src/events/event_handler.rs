use crate::events::Observer;
use crate::events::Event;
use std::collections::VecDeque;
use std::vec::Vec;

pub struct EventHandler<'a> {
    event_queue: VecDeque<Event>,
    observers: Vec<&'a dyn Observer>
}

impl<'a> EventHandler<'a> {
    pub fn new() -> EventHandler<'a> {
        EventHandler {
            event_queue: VecDeque::new(),
            observers: Vec::new()
        }
    }

    pub fn subscribe(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }

    pub fn unsubscribe(&mut self, observer: &'a dyn Observer) {
        let index = self.observers.iter().position(|&x| std::ptr::eq(x, observer)).unwrap();
        self.observers.remove(index);
    }

    pub fn queue_event(&mut self, event: Event) {
        self.event_queue.push_front(event);
    }

    pub fn get_event_queue(&self) -> &VecDeque<Event> {
        return &self.event_queue;
    }

    pub fn get_observers(&self) -> &Vec<&'a dyn Observer> {
        return &self.observers;
    }
}