use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use crate::render::pipeline::DisplayList;

/// Structure combining a [JoinHandle] for an executing thread, along with
/// channel information to communicate with the thread
pub struct AppThread<Message, Result> {
    pub handle: JoinHandle<Result>,
    /// One end of a channel to send messages of a specific type to the contained thread
    pub sink: Sender<Message>,
}

/// Struct to hold global application state etc...
pub struct AppThreads<'a> {
    /// The current rendering thread
    pub renderer: AppThread<DisplayList<'a>, ()>,
}
