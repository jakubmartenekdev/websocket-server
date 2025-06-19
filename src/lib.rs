use std::{pin::{Pin}, sync::Arc};

use tokio::{sync::{mpsc::{self}, Mutex}, task::JoinHandle};


pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

type Job = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel(size);
        let receiver = Arc::new(Mutex::new(rx));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            //dbg!("creating new worker thread!");
            workers.push(Worker::new(id, Arc::clone(&receiver)));
            // create some workers and store them in the vector
        }
        

        Self { workers, tx }        
    }

    pub async fn execute<F>(&self, f: F)
        where
            F: Future<Output = ()> + Send + 'static,
        {
            let job = Box::pin(f);
            //Pin::from()
            self.tx.send(job).await.expect("execute error");
    }
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self{
        let thread = tokio::spawn(async move {
            // dbg!("spawning new thread!");
            loop {
                let mut rx_guard = rx.lock().await;
                let job = rx_guard.recv().await;

                    println!("Worker {id} LOOPP");
                if let Some(job) = job {
                    println!("Worker {id} got a job; executing.");
                    job.await;
                } else {
                    break;
                }
            }
        });

        Worker { id, thread }   
    }
}