// trait Task {
//     fn run<T>(&self, input: T) -> T;

use core::task;
use std::future;

// }
use chrono::Utc;
use http::status::StatusCode;
use longrun::{InMemoryStateStore, StateStore, Task};
use paginate::Pages;
use uuid::Uuid;

static IMPLEMENTATION_VERSION: i32 = 0;

mod longrun {
    use core::fmt;
    // use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use std::task;
    use std::{collections::HashMap, env};

    use crate::IMPLEMENTATION_VERSION;
    #[derive(Clone)]
    pub enum States {
        Pending,
        Running,
        Completed,
        Failed,
        Timeout,
        Canceled,
    }

    #[derive(Clone)]
    pub struct Error {
        message: String,
        preferred_status: States,
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Error: {}", self.message)
        }
    }

    pub trait StateStore<T>
    where
        T: Clone + Send + Sync,
    {
        async fn get_state(&self, key: String) -> T;
        async fn set_state(&self, key: String, value: T);
        async fn new_task(&self, name: String, description: String) -> Task<T>;

        async fn load_task(&self, id: uuid::Uuid) -> Task<T>;
        async fn get_tasks(&self) -> Vec<Task<T>>;
    }

    pub struct InMemoryStateStore<T> {
        tasks: HashMap<uuid::Uuid, Task<T>>,
        variables: HashMap<String, T>,
    }

    impl<T> InMemoryStateStore<T> {
        pub async fn new() -> InMemoryStateStore<T> {
            InMemoryStateStore {
                tasks: HashMap::new(),
                variables: HashMap::new(),
            }
        }
    }

    impl<T> StateStore<T> for InMemoryStateStore<T>
    where
        T: Clone + Send + Sync,
    {
        async fn get_state(&self, key: String) -> T {
            // return self.variables.get(key).unwrap();
            //create future
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            return self.variables.get(&key).unwrap().clone();
        }

        async fn set_state(&self, key: String, value: T) {
            todo!()
        }

        async fn new_task(&self, name: String, description: String) -> Task<T> {
            todo!()
        }

        async fn load_task(&self, id: uuid::Uuid) -> Task<T> {
            todo!()
        }

        async fn get_tasks(&self) -> Vec<Task<T>> {
            todo!()
        }
    }

    // pub struct value
    pub struct Task<T> {
        name: String,
        id: uuid::Uuid,
        description: String,
        status: States,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        execute: fn() -> T,
        implementation: Implementation,
    }

    pub struct Implementation {
        spec_version: i32,
        operating_system: String,
        language: String,
        version: i32,
    }

    impl Implementation {
        pub fn new() -> Implementation {
            Implementation {
                spec_version: 0,
                language: env::consts::ARCH.to_string(),
                version: 0,
                operating_system: env::consts::OS.to_string(),
            }
        }
    }

    impl<T> Task<T> {
        pub fn new(name: String, description: String, run: fn() -> T) -> Task<T> {
            Task {
                name: name,
                id: uuid::Uuid::new_v4(),
                description: description,
                status: States::Pending,
                start_time: chrono::Utc::now(),
                end_time: chrono::Utc::now(),
                execute: run,
                implementation: Implementation::new(),
            }
        }
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        pub async fn run(&mut self) -> T {
            self.status = States::Running;

            let result: Result<T, Error> = || -> Result<T, Error> {
                let return_value = (self.execute)();
                self.status = States::Completed;
                Ok(return_value)
            }();

            self.status = match result {
                Ok(_) => States::Completed,
                Err(_) => States::Failed,
            };

            return result.unwrap();
        }
    }
}

async fn doit() {
    let store: InMemoryStateStore<i32> = longrun::InMemoryStateStore::new().await;

    let mut task: Task<i32> = Task::new("Task1".to_string(), "This is a task".to_string(), || {
        println!("Task1");
        return 1;
    });

    //print tasks
    let tasks = store.get_tasks().await;
    for task in tasks {
        println!("Task: {}", task.get_name());
    }

    let x = task.run().await;
    println!("Task1 returned: {}", x);
}
fn main() {
    // Task
    futures::executor::block_on(doit());
}
