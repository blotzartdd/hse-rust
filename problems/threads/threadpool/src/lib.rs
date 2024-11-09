#![forbid(unsafe_code)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub enum Message {
    NewJob,
    Terminate,
}

pub struct ThreadTask {
    // TODO: your code goes here
}

impl ThreadTask {
    pub fn new<Job>(id: usize, receiver: todo!()) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

pub struct ThreadPool<Job> {
    tasks: Vec<ThreadTask>,
    sender: mpsc::Sender<Message<Job>>,
}

impl<Job> ThreadPool<Job> {
    pub fn new(size: usize) -> ThreadPool<Job> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn execute(&self, job: Job) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<Job> Drop for ThreadPool<Job> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}
