use std::time::Instant;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Job {
    pub priority: i32,
    pub added_at: Instant,
    pub data: String,
}

impl Job {
   pub  fn new(priority: i32, data: String) -> Self {
        Job {
            priority,
            data,
            added_at: Instant::now(),
        }
    }
}

// Implement a comparator for Job priority
impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => {
                self.added_at.cmp(&other.added_at).reverse()
            },
            order => {
               order
            }
        }
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
