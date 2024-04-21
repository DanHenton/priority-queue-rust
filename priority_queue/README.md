# Implement a Priority Queue
Implement a Priority Queue in Rust using a BinaryHeap and a comparator on the Job struct.

## Usage
This module could be useful when need it comes improving the performance of API response times by utilizing backend workers.
The Priority Queue is thread safe so that clients and workers can work concurrently safely. 


```rust
let queue = PriorityQueue::new();

let priority = 10;
let job = Job::new(priority, String::from("job-one"));

queue.push(job);
queue.size();
// => 1

queue.pop()
// => Some(Job { priority: 10, added_at: Instant { tv_sec: 1266214, tv_nsec: 684697083 }, data: "job-one" })
```

## Performance
| push   | pop          | peek/peek_mut |
|--------|--------------|---------------|
| O (1)~ | O (log( n )) | O (1)         |

based on BinaryHeap.

## Test
To run the tests:

```
cargo test
```