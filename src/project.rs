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

    let task_rx = Arc::new(Mutex::new(task_rx));

    for i in 0..4 {
        let task_rx = Arc::clone(&task_rx);
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            loop {
                let task = task_rx.lock().unwrap().recv();
                match task {
                    Ok(task) => {
                        let start = Instant::now();

                        let task_result = match task {
                            Task::Compute {id, iterations} => {
                                let result = process_compute(id, iterations);
                                let duration_ms = start.elapsed().as_millis();
                                match result {
                                    Ok(msg) => TaskResult::Success {
                                        id,
                                        task_type: "compute".to_string(),
                                        duration_ms
                                    },
                                    Err(msg) => TaskResult::Error {
                                        id,
                                        message: msg
                                    }
                                }
                            },
                            Task::Download {id, url} => {
                                let result = process_download(id, &url);
                                let duration_ms = start.elapsed().as_millis();
                                match result {
                                    Ok(msg) => TaskResult::Success {
                                        id,
                                        task_type: "download".to_string(),
                                        duration_ms
                                    },
                                    Err(msg) => TaskResult::Error {
                                        id,
                                        message: msg
                                    }
                                }                            
                            },
                            Task::Process {id, data} => {
                                let result = process_data(id, data);
                                let duration_ms = start.elapsed().as_millis();
                                match result {
                                    Ok(msg) => TaskResult::Success {
                                        id,
                                        task_type: "process".to_string(),
                                        duration_ms
                                    },
                                    Err(msg) => TaskResult::Error {
                                        id,
                                        message: msg
                                    }
                                }                            
                            }
                        };

                        result_tx.send(task_result).unwrap();
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });
    }

    // TODO: Main thread:
    //   1. Sends all tasks to task_tx
    //   2. Receives results from result_rx
    //   3. Prints results as they arrive
    //   4. Prints final statistics

    for task in tasks {
        task_tx.send(task).unwrap();
    }
    drop(task_tx);
    drop(result_tx);

    for task_result in result_rx {
        match task_result {
            TaskResult::Success {id, task_type, duration_ms} => {
                println!("✓ Task {} ({}) completed in {}ms", id, task_type, duration_ms);
                let mut stats_guard = stats.lock().unwrap();
                stats_guard.tasks_completed += 1;
                stats_guard.total_duration_ms += duration_ms;
            },
            TaskResult::Error {id, message} => {
                println!("✗ Task {} failed: {}", id, message);
                let mut stats_guard = stats.lock().unwrap();
                stats_guard.tasks_failed += 1;
            }
        }
    }
    let final_stats = stats.lock().unwrap();
    println!("\n=== Final Statistics ===");
    println!("Tasks completed: {}", final_stats.tasks_completed);
    println!("Tasks failed: {}", final_stats.tasks_failed);
    println!("Total duration: {}ms", final_stats.total_duration_ms);
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
