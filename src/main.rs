// trait Task {
//     fn run<T>(&self, input: T) -> T;

use core::task;

// }
use chrono::Utc;
use http::status::StatusCode;
use longrun::{StateStore, Task};
use uuid::Uuid;
use paginate::Pages;

mod longrun {
    use std::collections::HashMap;
    pub enum States { 
        Pending,
        Running,
        Completed,
        Failed,
        Timeout,
        Cancelled,
    }

    pub trait StateStore {
        fn get_state<T>(&self, key: String) -> T;
        fn set_state<T>(&self, key: String, value: T);
        fn new_task<T>(&self, name: String, description: String) -> Task;
        fn load_task<T>(&self, id: uuid::Uuid) -> Task;
    }

    pub struct InMemoryStateStore {
        tasks: HashMap<uuid::Uuid, Task>,
    }

    impl InMemoryStateStore {
        pub fn new() -> InMemoryStateStore {
            InMemoryStateStore {
                tasks: HashMap::new(),
            }
        }
    }

    impl StateStore for InMemoryStateStore {
        fn get_state<T>(&self, key: String) -> T {
            todo!()
        }

        fn set_state<T>(&self, key: String, value: T) {
            todo!()
        }

        fn new_task<T>(&self, name: String, description: String) -> Task {
            todo!()
        }

        fn load_task<T>(&self, id: uuid::Uuid) -> Task {
            todo!()
        }
    }gi

    // pub struct value
    pub struct Task {
        name: String,
        id: uuid::Uuid,
        description: String,
        status: http::status::StatusCode,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        run: fn(),
        // pub fn run<T>(&self, input: T) -> T {
        //     // Task
        // }
    }

    impl Task {
        pub fn new(name: String, description: String, run: fn()) -> Task {
            Task {
                name: name,
                id: uuid::Uuid::new_v4(),
                description: description,
                status: http::status::StatusCode::OK,
                start_time: chrono::Utc::now(),
                end_time: chrono::Utc::now(),
                run: run,
            }
        }
    }
}
fn main() {
    // Task
    let store = longrun::InMemoryStateStore::new();

    // let task = store.new_task("Task1".to_string());

    let task: Task = Task::new(
        "Task1".to_string(),
        "This is a task".to_string(),
        || {
            println!("Task1");
        },
    );
}
