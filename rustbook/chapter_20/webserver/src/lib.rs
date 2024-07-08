use std::thread;
use std::sync::{mpsc, Arc, Mutex};



type Job = Box<dyn FnBox + Send + 'static>;



enum Message {
    NewJob(Job),
    Terminate,
}



trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}



struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> Self
    {
        Self {
            id,
            handle: Some(thread::spawn(move || loop {
                let msg = receiver.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {id} got a job.");
                        job.call_box();
                    },
                    Message::Terminate => break,
                };
            })),
        }
    }
}



pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Create a new thread pool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> Self {
        assert!(size > 0);
        
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

		Self {
            workers,
            sender: tx
        }
	}


	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static
	{
		self.sender.send(Message::NewJob(Box::new(f))).unwrap();
	}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join();
            }
        }
    }
}