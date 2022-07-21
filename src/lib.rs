pub struct ThreadPool;

impl ThreadPool {
    pub fn new(n_threads: usize) -> ThreadPool  {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}