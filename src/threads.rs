use crate::render::display_lists::DisplayList;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

/// Structure combining a [JoinHandle] for an executing thread, along with
/// channel information to communicate with the thread
#[derive(Debug)]
pub struct AppThread<Message, Result> {
    /// The [JoinHandle] used to wait for thread completion/cancellation etc...
    pub handle: Option<JoinHandle<Result>>,
    /// One end of a channel to send messages of a specific type to the contained thread
    pub sink: Sender<Message>,
}

impl<Message, Result> AppThread<Message, Result> {
    pub fn join(&mut self) {
        self.handle.take().unwrap().join().expect("Failed to join");
    }
}

/// Struct to hold global application state etc...
#[derive(Debug)]
pub struct AppThreads {
    /// The current rendering thread
    pub renderer: AppThread<DisplayList, ()>,
}
