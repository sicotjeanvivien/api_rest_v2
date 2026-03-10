use async_trait::async_trait;

use crate::{
    domain::task::{
        model::{NewTask, Task, UpdateTask},
        repository::TaskRepository,
    },
    errors::repository_error::RepositoryError,
};
use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicI32, Ordering},
};

pub struct InMemoryTaskStore {
    tasks: Arc<Mutex<Vec<Task>>>,
    next_id: AtomicI32,
}

impl InMemoryTaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(vec![])),
            next_id: AtomicI32::new(1),
        }
    }

    fn lock_tasks(&self) -> Result<MutexGuard<'_, Vec<Task>>, RepositoryError> {
        self.tasks.lock().map_err(|_| RepositoryError::Internal)
    }

    fn get_task(&self, id: i32) -> Result<Task, RepositoryError> {
        let tasks = self.lock_tasks()?;
        tasks
            .iter()
            .find(|t| t.id() == id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }
}

#[async_trait]
impl TaskRepository for InMemoryTaskStore {
    async fn get(&self, id: i32) -> Result<Task, RepositoryError> {
        self.get_task(id)
    }

    async fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let tasks = self.lock_tasks()?;
        Ok(tasks.clone())
    }

    async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        println!("create");
        let mut tasks: MutexGuard<'_, Vec<Task>> = self.lock_tasks()?;
        let id: i32 = self.next_id.fetch_add(1, Ordering::SeqCst);
        let task: Task = Task::new(id, new_task.title, new_task.description, false);
        tasks.push(task);
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let mut tasks = self.lock_tasks()?;
        let index = tasks
            .iter()
            .position(|t| t.id() == id)
            .ok_or(RepositoryError::NotFound)?;
        tasks.remove(index);
        Ok(())
    }

    async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        let mut tasks = self.lock_tasks()?;
        let task = tasks
            .iter_mut()
            .find(|t| t.id() == update_task.id)
            .ok_or(RepositoryError::NotFound)?;
        if let Some(title) = update_task.title {
            task.set_title(title);
        }
        if let Some(description) = update_task.description {
            task.set_description(Some(description));
        }
        if let Some(done) = update_task.done {
            task.set_done(done);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::model::NewTask;

    #[test]
    fn test_create_and_get_all() {
        let store = build_store();
        let tasks = store.get_all().unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title(), "test create task 0");
    }

    #[test]
    fn test_create() {
        let store = InMemoryTaskStore::new();
        let new_task = NewTask {
            title: String::from("test create task"),
            description: Some(String::from("description")),
        };
        store.create(new_task).unwrap();
        assert_eq!(store.get_all().unwrap().len(), 1);
    }

    #[test]
    fn test_get_all() {
        let store = build_store();
        let tasks = store.get_all().unwrap();

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title(), "test create task 0");
        assert_eq!(tasks[0].description().unwrap(), "description");
        assert_eq!(tasks[1].title(), "test create task 1");
        assert_eq!(tasks[1].description(), None);
    }

    #[test]
    fn test_get() {
        let store = build_store();

        assert_eq!(store.get(1).unwrap().title(), "test create task 1");
        assert_eq!(store.get(34).unwrap_err(), RepositoryError::NotFound);
    }

    #[test]
    fn test_delete() {
        let store = build_store();

        assert_eq!(store.delete(1).unwrap(), ());
        assert_eq!(store.get(1).unwrap_err(), RepositoryError::NotFound);
        assert_eq!(store.delete(34).unwrap_err(), RepositoryError::NotFound);
    }

    #[test]
    fn test_update() {
        let store = build_store();
        let upade_task = UpdateTask {
            id: 1,
            title: Some(String::from("update task 1")),
            description: None,
            done: Some(true),
        };
        let update_task_bad_id = UpdateTask {
            id: 34,
            title: None,
            description: None,
            done: Some(true),
        };
        assert_eq!(store.update(upade_task).unwrap(), ());
        assert_eq!(
            store.update(update_task_bad_id).unwrap_err(),
            RepositoryError::NotFound
        );
        let task = store.get(1).unwrap();
        assert_eq!(task.title(), "update task 1");
        assert_eq!(task.done(), true);
    }

    fn build_store() -> InMemoryTaskStore {
        let store = InMemoryTaskStore::new();
        let new_task_1 = NewTask {
            title: String::from("test create task 0"),
            description: Some(String::from("description")),
        };
        let new_task_2 = NewTask {
            title: String::from("test create task 1"),
            description: None,
        };
        store.create(new_task_1).unwrap();
        store.create(new_task_2).unwrap();
        store
    }
}
