#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};
    use crate::{PriorityQueue, Job};

  #[test]
  fn test_push() {
    let queue = PriorityQueue::new();

    let job = Job::new(10, String::from("job-one"));

    queue.push(job);
    assert_eq!(queue.size(), 1);
  }

  #[test]
  fn test_pop() {
    let queue = PriorityQueue::new();

    let job = Job::new(10, String::from("job-one"));

    queue.push(job.clone());

    assert_eq!(queue.pop(), Some(job));
  }

  #[test]
  fn test_empty_pop() {
    let queue = PriorityQueue::new();

    assert_eq!(queue.pop(), None);
  }

  #[test]
  fn test_pop_priority() {
    let queue = PriorityQueue::new();

    let priority = 10;
    let job_one = Job::new(priority, String::from("job-one"));
    let job_two= Job::new(priority - 1, String::from("job-two"));
    let job_three= Job::new(priority + 1, String::from("job-three"));

    queue.push(job_one.clone());
    queue.push(job_two.clone());
    queue.push(job_three.clone());

    assert_eq!(queue.pop(), Some(job_three));
    assert_eq!(queue.pop(), Some(job_one));
    assert_eq!(queue.pop(), Some(job_two));
  }

  #[test]
  fn test_retain_order_when_matching_priorities() {
    let queue = PriorityQueue::new();

    let priority = 10;
    let job_one = Job::new(priority, String::from("job-one"));
    let job_two= Job::new(priority, String::from("job-two"));

    queue.push(job_one.clone());
    queue.push(job_two.clone());

    assert_eq!(queue.pop(), Some(job_one)); 
    assert_eq!(queue.pop(), Some(job_two));
  }

  #[test]
  fn concurrent_workers() {
    let queue = Arc::new(PriorityQueue::new());
    let mut threads = vec![];

    for index in 0..10 {
      let thread_queue = queue.clone();
      let priority = if index % 2 == 0 { 1 } else { 2 };

        let job = Job::new(priority, index.to_string());
        thread_queue.push(job);

      threads.push(thread::spawn(move || {
        for i in 0..1_000 {
        let job = Job::new(priority, format!("{}-{}", index.to_string(), i.to_string()));
        thread_queue.push(job)
        }
      }));
    }

    for thread in threads {
      thread.join().unwrap();
    }

    let mut popped_jobs = vec![];

    loop {
      let job = queue.acquire_lock().pop();

      if job.is_none() {
        break;
      }
      popped_jobs.push(job.unwrap());
    }

    // Assert that higher priority tasks were popped first
    for i in 0..popped_jobs.len() - 1 {
      assert!(popped_jobs[i].priority >= popped_jobs[i + 1].priority);
    }
  }
}