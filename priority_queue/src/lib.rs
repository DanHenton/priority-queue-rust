use std::collections::BinaryHeap;
use std::sync::{Mutex, MutexGuard};

mod test;

mod job;
pub use job::Job;

/**
  Implement a PriorityQueue which stores Jobs

  The queue will priorities the jobs with a higher priority coming before a lower one.
  When there are multiple Jobs with the same priority ensure that oldest Job is prioritised first.

  This PriorityQueue is thread safe
 */
pub struct PriorityQueue {
    queue: Mutex<BinaryHeap<Job>>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            queue: Mutex::new(BinaryHeap::new()),
        }
    }

    pub fn size(&self) -> usize {
        let queue_lock = self.acquire_lock();

        queue_lock.len()
    }

    /**
     Push a job onto the queue
     */
    pub fn push(&self, job: Job) {
        let mut queue_lock = self.acquire_lock();

        queue_lock.push(job)
    }

    /**
     Pop a Job off the queue in priority order of:
     1. Highest priority Job
     2. Oldest Job in with the highest priority
     */
    pub fn pop(&self) -> Option<Job> {
        let mut queue_lock = self.acquire_lock();

        queue_lock.pop()
    }

    // Acquire a lock on queue
    fn acquire_lock(&self) -> MutexGuard<'_, BinaryHeap<Job>> {
        let queue_lock = match self.queue.try_lock() {
            Ok(lock) => lock,
            Err(_) => self.queue.lock().unwrap(),
        };

        queue_lock
    }
}