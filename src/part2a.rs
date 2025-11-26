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

    for t in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let r = process_task(t);
            tx.send(r).unwrap();
        });
    }

    drop(tx);

    for r in rx {
        match r {
            TaskResult::Success {result, ..} => {
                println!("{result}");
            },
            TaskResult::Error {error, ..} => {
                println!("{error}");
            }
        }
    }

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
