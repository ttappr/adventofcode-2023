#![allow(unused)]

use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() -> usize + Send + 'static>;

pub struct ThreadPool {
    workers  : Vec<Worker>,
    sender   : Option<Sender<Job>>,
    receiver : Receiver<usize>,
}
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        let (sender, th_receiver) = channel();
        let (th_sender, receiver) = channel();

        let th_receiver = Arc::new(Mutex::new(th_receiver));

        for id in 0..size {
            workers.push(Worker::new(id, 
                                     th_receiver.clone(), 
                                     th_sender.clone()));
        }
        Self { workers, sender: Some(sender), receiver }
    }
    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() -> usize + Send + 'static
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {

            println!("Shutting down worker {}.", worker.id);

            if let Some(thread) = worker.h_thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id       : usize,
    h_thread : Option<JoinHandle<()>>,
}
impl Worker {
    pub fn new(id       : usize, 
               receiver : Arc<Mutex<Receiver<Job>>>,
               sender   : Sender<usize>)
        -> Self 
    {
        let h_thread = thread::spawn(move || loop { 
            let message = receiver.lock().unwrap().recv();

            println!("Worker {} executing job.", id);
            
            match message {
                Ok(job) => sender.send(job()).unwrap(),
                Err(_)  => break,
            }
        });
        Self { id, h_thread: Some(h_thread) }
    }
}
