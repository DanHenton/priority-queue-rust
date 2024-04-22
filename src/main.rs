use priority_queue::*;


fn main() {
    let queue =  PriorityQueue::new();

    let job = Job::new( 10, String::from("Data One!"));
    let job2 = Job::new( 10, String::from("Data Two!"));

    queue.push(job);
    queue.push(job2);

    println!("pop: {:?}", queue.pop());

    println!("Queue size: {}", queue.size());
}
