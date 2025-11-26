use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    work_duration: u64,
}

#[derive(Debug)]
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

    let mut handles = vec![];

    for task in tasks {
        let stats = Arc::clone(&stats);
        let handle = thread::spawn(move || {
            process_task(task, stats);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_stats = stats.lock().unwrap();
    println!("Final Statistics:");
    println!("  Completed: {}", final_stats.completed);
    println!("  Failed: {}", final_stats.failed);
    println!("  Total time: {}ms", final_stats.total_time_ms);
}

fn process_task(task: Task, stats: Arc<Mutex<Stats>>) {
    println!("Processing task {}", task.id);

    let start = std::time::Instant::now();
    thread::sleep(Duration::from_millis(task.work_duration));
    let duration = start.elapsed().as_millis() as u64;

    // TODO: Lock the mutex and update stats
    // Handle simulated failures (e.g., if id % 5 == 0)

    let mut stats_guard = stats.lock().unwrap();
    if task.id % 5 == 0 {
        stats_guard.failed += 1;
    } else {
        stats_guard.completed += 1;
    }
    stats_guard.total_time_ms += duration;
}
