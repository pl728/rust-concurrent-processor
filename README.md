# Concurrent Task Processor Lab

## Project Overview
Build a system that processes tasks using multiple threads, combining thread spawning, message passing, and shared state. This is a hands-on project to practice Rust's fearless concurrency concepts.

---

## Project Structure

Organize your project with separate files for each part:

```
src/
├── main.rs          # Runs all parts
├── part1.rs         # Basic threads + join handles
├── part2.rs         # Message passing with channels
├── part3.rs         # Shared state with Mutex + Arc
└── full_project.rs  # Complete integrated system
```

**main.rs:**
```rust
mod part1;
mod part2;
mod part3;
mod full_project;

fn main() {
    println!("=== Part 1: Basic Threads ===");
    part1::run();
    
    println!("\n=== Part 2: Message Passing ===");
    part2::run();
    
    println!("\n=== Part 3: Shared State ===");
    part3::run();
    
    println!("\n=== Full Project ===");
    full_project::run();
}
```

---

## Part 1: Basic Thread Pool (Threads + Join Handles)

**File:** `src/part1.rs`

Create a simple task processor that spawns worker threads.

```rust
use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    work_duration: u64,
}

pub fn run() {
    let tasks = vec![
        Task { id: 1, work_duration: 100 },
        Task { id: 2, work_duration: 200 },
        Task { id: 3, work_duration: 150 },
    ];
    
    // TODO: Spawn a thread for each task
    // TODO: Use join handles to wait for completion
    // TODO: Print results
}

fn process_task(task: Task) {
    println!("Starting task {}", task.id);
    thread::sleep(Duration::from_millis(task.work_duration));
    println!("Completed task {}", task.id);
}
```

**Goals:**
- ✅ Spawn one thread per task using `thread::spawn`
- ✅ Use `move` closure to transfer ownership
- ✅ Collect join handles in a vector
- ✅ Wait for all threads to complete with `.join()`

**Expected Output:**
```
Starting task 1
Starting task 2
Starting task 3
Completed task 1
Completed task 3
Completed task 2
```
*(Order may vary due to concurrency)*

---

## Part 2: Message Passing System (Channels)

**File:** `src/part2.rs`

Extend the processor to use channels for worker-to-main communication.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    work_duration: u64,
}

enum TaskResult {
    Success { id: u32, result: String },
    Error { id: u32, error: String },
}

pub fn run() {
    let (tx, rx) = mpsc::channel();
    
    let tasks = vec![
        Task { id: 1, work_duration: 100 },
        Task { id: 2, work_duration: 200 },
        Task { id: 3, work_duration: 150 },
        Task { id: 4, work_duration: 50 },
    ];
    
    // TODO: Spawn worker threads that process tasks
    // TODO: Each worker sends TaskResult through the channel
    // TODO: Main thread receives and prints results
    
    // Hint: Clone tx for each thread, or pass ownership carefully
}

fn process_task(task: Task) -> TaskResult {
    println!("Processing task {}", task.id);
    thread::sleep(Duration::from_millis(task.work_duration));
    
    // Simulate occasional failures
    if task.id % 5 == 0 {
        TaskResult::Error {
            id: task.id,
            error: "Task failed".to_string(),
        }
    } else {
        TaskResult::Success {
            id: task.id,
            result: format!("Task {} completed", task.id),
        }
    }
}
```

**Goals:**
- ✅ Workers send results back via `tx.send()`
- ✅ Main thread receives with `rx.recv()` or iterates over `rx`
- ✅ Handle both success and error cases
- ✅ Use `tx.clone()` for multiple producers

**Challenge:** Process 10+ tasks with only 3 worker threads (you'll need to share the channel receiver or use a different pattern).

---

## Part 3: Shared Counter (Mutex + Arc)

**File:** `src/part3.rs`

Add shared state to track statistics across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    work_duration: u64,
}

struct Stats {
    completed: u32,
    failed: u32,
    total_time_ms: u64,
}

impl Stats {
    fn new() -> Self {
        Stats {
            completed: 0,
            failed: 0,
            total_time_ms: 0,
        }
    }
}

pub fn run() {
    let stats = Arc::new(Mutex::new(Stats::new()));
    
    let tasks = vec![
        Task { id: 1, work_duration: 100 },
        Task { id: 2, work_duration: 200 },
        Task { id: 3, work_duration: 150 },
        Task { id: 4, work_duration: 80 },
        Task { id: 5, work_duration: 120 },
    ];
    
    // TODO: Spawn threads that share the stats
    // TODO: Each thread updates stats after processing
    // TODO: Print final statistics
    
    // Hint: Clone the Arc for each thread
}

fn process_task(task: Task, stats: Arc<Mutex<Stats>>) {
    println!("Processing task {}", task.id);
    
    let start = std::time::Instant::now();
    thread::sleep(Duration::from_millis(task.work_duration));
    let duration = start.elapsed().as_millis() as u64;
    
    // TODO: Lock the mutex and update stats
    // Handle simulated failures (e.g., if id % 5 == 0)
}
```

**Goals:**
- ✅ Share `Stats` across multiple threads using `Arc<Mutex<T>>`
- ✅ Lock the mutex to safely update statistics
- ✅ Avoid data races
- ✅ Verify final counts are correct

**Expected Output:**
```
Processing task 1
Processing task 2
...
Final Statistics:
  Completed: 4
  Failed: 1
  Total time: 650ms
```

---

## Full Project: Complete Task Processing System

**File:** `src/full_project.rs`

Combine everything into a realistic concurrent task processor.

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Task types
#[derive(Clone, Debug)]
enum Task {
    Compute { id: u32, iterations: u32 },
    Download { id: u32, url: String },
    Process { id: u32, data: Vec<u32> },
}

// Results
#[derive(Debug)]
enum TaskResult {
    Success { id: u32, task_type: String, duration_ms: u128 },
    Error { id: u32, message: String },
}

// Shared statistics
struct SystemStats {
    tasks_completed: u32,
    tasks_failed: u32,
    total_duration_ms: u128,
    active_workers: u32,
}

impl SystemStats {
    fn new() -> Self {
        SystemStats {
            tasks_completed: 0,
            tasks_failed: 0,
            total_duration_ms: 0,
            active_workers: 0,
        }
    }
}

pub fn run() {
    // Create 20 random tasks
    let tasks = generate_tasks(20);
    
    // Set up channels and shared state
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let stats = Arc::new(Mutex::new(SystemStats::new()));
    
    // TODO: Create 4 worker threads that:
    //   1. Receive tasks from task_rx (need to share receiver - use Arc<Mutex<Receiver>>)
    //   2. Process them
    //   3. Send results to result_tx
    //   4. Update shared stats
    
    // TODO: Main thread:
    //   1. Sends all tasks to task_tx
    //   2. Receives results from result_rx
    //   3. Prints results as they arrive
    //   4. Prints final statistics
}

// Helper functions to implement
fn generate_tasks(count: u32) -> Vec<Task> {
    use Task::*;
    let mut tasks = vec![];
    
    for i in 1..=count {
        let task = match i % 3 {
            0 => Compute { id: i, iterations: 1000 },
            1 => Download { id: i, url: format!("http://example.com/{}", i) },
            _ => Process { id: i, data: vec![1, 2, 3, 4, 5] },
        };
        tasks.push(task);
    }
    
    tasks
}

fn process_compute(id: u32, iterations: u32) -> Result<String, String> {
    thread::sleep(Duration::from_millis(50));
    Ok(format!("Computed {} iterations", iterations))
}

fn process_download(id: u32, url: &str) -> Result<String, String> {
    thread::sleep(Duration::from_millis(100));
    if id % 7 == 0 {
        Err("Download failed".to_string())
    } else {
        Ok(format!("Downloaded from {}", url))
    }
}

fn process_data(id: u32, data: Vec<u32>) -> Result<String, String> {
    thread::sleep(Duration::from_millis(75));
    let sum: u32 = data.iter().sum();
    Ok(format!("Processed {} items, sum: {}", data.len(), sum))
}
```

**Key Challenges:**
1. **Sharing the task receiver** - Multiple workers need to receive from the same channel. You'll need `Arc<Mutex<Receiver<Task>>>`
2. **Coordinating shutdown** - How do workers know when all tasks are done?
3. **Result collection** - Main thread needs to know when to stop receiving results

**Architecture:**
```
Main Thread
    │
    ├─ Sends tasks ──→ task_tx ──→ [Queue] ──→ Workers (shared rx)
    │
    └─ Receives ←──── result_tx ←──────────── Workers
    
Stats (Arc<Mutex<Stats>>) ← shared by all workers
```

---

## Challenges & Extensions

### Easy Extensions:
1. **Task Priority** - Add a priority field, process high-priority first
2. **Progress Indicator** - Print "X/Y tasks completed" as they finish
3. **Colored Output** - Use `colored` crate for better visualization
4. **Timeout** - Fail tasks that take too long

### Medium Extensions:
1. **Dynamic Worker Pool** - Add/remove workers based on queue length
2. **Task Retry Logic** - Retry failed tasks up to 3 times
3. **Rate Limiting** - Limit tasks processed per second
4. **Task Dependencies** - Some tasks must complete before others start

### Hard Extensions:
1. **Work Stealing** - Workers can steal from each other's queues
2. **Multiple Priority Queues** - Separate channels for different priorities
3. **Graceful Shutdown** - Send shutdown signal, workers finish current task
4. **Deadlock Simulation** - Create and then fix a deadlock scenario

---

## Testing Your Code

Create tests to verify correctness:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_counting() {
        // 10 threads each increment counter 100 times
        // Final count should be exactly 1000
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 1000);
    }

    #[test]
    fn test_message_passing() {
        // 5 threads send 20 messages each
        // Verify all 100 messages received
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        for i in 0..5 {
            let tx = tx.clone();
            let handle = thread::spawn(move || {
                for j in 0..20 {
                    tx.send(i * 20 + j).unwrap();
                }
            });
            handles.push(handle);
        }

        drop(tx); // Important! Drop original tx

        for handle in handles {
            handle.join().unwrap();
        }

        let received: Vec<_> = rx.iter().collect();
        assert_eq!(received.len(), 100);
    }
}
```

---

## Learning Objectives

By completing this project, you'll master:

✅ **Thread spawning** with `thread::spawn`  
✅ **Join handles** and waiting for thread completion  
✅ **Move closures** for ownership transfer  
✅ **Channels** (`mpsc`) for message passing  
✅ **Multiple producers** via `tx.clone()`  
✅ **Mutex** for protecting shared mutable state  
✅ **Arc** for shared ownership across threads  
✅ **Send/Sync traits** and their importance  
✅ **Avoiding data races** and deadlocks  
✅ **Real-world concurrent patterns**

---

## Getting Started

1. ✅ **Set up project structure** - Create all the files
2. ✅ **Complete Part 1** - Get comfortable with threads
3. ✅ **Complete Part 2** - Learn message passing
4. ✅ **Complete Part 3** - Practice shared state
5. ✅ **Build full project** - Combine everything
6. ✅ **Add extensions** - Push yourself further
7. ✅ **Write tests** - Verify correctness

---

## Tips & Common Pitfalls

**Thread Ownership:**
- Remember to use `move` when the thread needs to own data
- Clone `Arc` before moving into threads

**Channel Management:**
- Drop unused `tx` senders so `rx` iterator knows when to end
- Clone `tx` for multiple producers

**Mutex Usage:**
- Lock scopes should be as short as possible
- Be careful not to hold locks across `.await` or long operations

**Deadlock Prevention:**
- Always acquire locks in the same order
- Don't hold multiple locks if possible
- Release locks before sleeping or waiting

---

## Resources

- [The Rust Book Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [std::thread documentation](https://doc.rust-lang.org/std/thread/)
- [std::sync::mpsc documentation](https://doc.rust-lang.org/std/sync/mpsc/)
- [std::sync::Mutex documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [std::sync::Arc documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)

Good luck, and enjoy fearless concurrency! 🦀