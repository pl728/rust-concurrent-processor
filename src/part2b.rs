use std::sync::{mpsc, Arc, Mutex};
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
    // we need worker threads to be able to receive tasks
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    
    let task_rx = Arc::new(Mutex::new(task_rx));

    let tasks = vec![
        Task { id: 1, work_duration: 100 },
        Task { id: 2, work_duration: 200 },
        Task { id: 3, work_duration: 150 },
        Task { id: 4, work_duration: 50 },
        Task { id: 5, work_duration: 180 },
        Task { id: 6, work_duration: 90 },
        Task { id: 7, work_duration: 220 },
        Task { id: 8, work_duration: 130 },
        Task { id: 9, work_duration: 170 },
        Task { id: 10, work_duration: 60 },
    ];

    // TODO: Spawn worker threads that process tasks
    // TODO: Each worker sends TaskResult through the channel
    // TODO: Main thread receives and prints results

    for i in 0..3 {
        let task_rx = Arc::clone(&task_rx);
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            loop {
                let task = task_rx.lock().unwrap().recv();
                match task {
                    Ok(task) => {
                        let result = process_task(task);
                        result_tx.send(result).unwrap();
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });
    }

    for t in tasks {
        task_tx.send(t).unwrap();
    }
    drop(task_tx);
    drop(result_tx);

    for result in result_rx {
        match result {
            TaskResult::Success {id, result} => {
                println!("{result}");
            },
            TaskResult::Error{id, error} => {
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
