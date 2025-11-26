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

    let mut handles = vec![];

    for task in tasks {
        let handle = thread::spawn(move || {
            process_task(task);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn process_task(task: Task) {
    println!("Starting task {}", task.id);
    thread::sleep(Duration::from_millis(task.work_duration));
    println!("Completed task {}", task.id);
}