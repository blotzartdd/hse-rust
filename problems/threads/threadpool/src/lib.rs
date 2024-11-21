#![forbid(unsafe_code)]
use std::mem;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub enum Message<Job> {
    NewJob(Job),
    Terminate,
}

pub struct ThreadTask<Job> {
    id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Message<Job>>>>,
    cur_thread: thread::JoinHandle<()>,
}

impl<Job: Send + 'static + FnOnce()> ThreadTask<Job> {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<Job>>>>) -> ThreadTask<Job> {
        let thread_name = format!("Task {id}");

        let receiver_clone = Arc::clone(&receiver);
        let cur_thread = thread::Builder::new()
            .name(thread_name)
            .spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                match job {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            })
            .unwrap();

        ThreadTask {
            id,
            receiver: receiver_clone,
            cur_thread,
        }
    }

    pub fn join_thread(self) {
        self.cur_thread.join().unwrap();
    }
}

pub struct ThreadPool<Job: Send + 'static + FnOnce()> {
    tasks: Vec<ThreadTask<Job>>,
    sender: mpsc::Sender<Message<Job>>,
}

impl<Job: Send + 'static + FnOnce()> ThreadPool<Job> {
    pub fn new(size: usize) -> ThreadPool<Job> {
        let mut tasks: Vec<ThreadTask<Job>> = Vec::new();
        let (sender, receiver): (mpsc::Sender<Message<Job>>, mpsc::Receiver<Message<Job>>) =
            mpsc::channel();
        let mutex_receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let new_thread_task = ThreadTask::new(id, Arc::clone(&mutex_receiver));

            let tmp = ThreadTask {
                id: new_thread_task.id,
                receiver: new_thread_task.receiver,
                cur_thread: new_thread_task.cur_thread,
            };

            tasks.push(tmp);
        }

        ThreadPool { tasks, sender }
    }

    pub fn execute(&self, job: Job) {
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl<Job: Send + 'static + FnOnce()> Drop for ThreadPool<Job> {
    fn drop(&mut self) {
        for _ in 0..self.tasks.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        let tasks = mem::take(&mut self.tasks);
        for task in tasks {
            task.join_thread();
        }
    }
}
