use std::{
    sync::{
        mpsc::{
            channel,
            Sender,
            Receiver,
        },
        Arc,
        Mutex,
    },
    thread
};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let mut threads: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            //NOTE: What I came up with
            //let mut worker: Worker = Worker::new(id);
            //threads[id] = worker;

            //NOTE: The book's suggestion
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }
        return ThreadPool {
            threads: threads,
            sender: sender,
        };
    }

    pub fn execute<F> (&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    handler: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        return Worker {
            id: id,
            handler: thread::spawn(move || loop {
                /* NOTE: A while let loop is not used here because
                 *       it does not work as expected due to the
                 *       lifetime of the MutexGuard<T> returned by
                 *       lock(), which determines ownership of the
                 *       lock. The lock would be held for the
                 *       duration of the loop iteration instead,
                 *       which is not desirable.
                 */
                let job: Job = receiver.lock()
                    .expect("Could not acquire lock...")
                    .recv()
                    .unwrap();
                println!("Worker {id} got a job; executing.");
                job();  //NOTE: job is a closure, hence the "()"
            }),
        };
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
