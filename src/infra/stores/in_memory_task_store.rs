use crate::{
    domain::task::{
        model::{NewTask, Task, UpdateTask},
        repository::TaskRepository,
    },
    errors::repository_error::RepositoryError,
};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct InMemoryTaskStore {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl InMemoryTaskStore {
    pub fn new() -> Self {
        InMemoryTaskStore {
            tasks: Arc::new(Mutex::new(vec![])),
        }
    }

    fn lock_tasks(&self) -> Result<MutexGuard<'_, Vec<Task>>, RepositoryError> {
        let tasks = self.tasks.lock().map_err(|_| RepositoryError::Internal)?;
        Ok(tasks)
    }
    fn get_task(&self, id: u32) -> Result<Task, RepositoryError> {
        let tasks = self.lock_tasks()?;
        tasks
            .iter()
            .find(|t| t.id() == id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }
    fn get_task_index(&self, id: u32) -> Result<usize, RepositoryError> {
        let tasks = self.lock_tasks()?;
        match tasks.iter().position(|task| task.id() == id) {
            Some(i) => Ok(i),
            None => Err(RepositoryError::NotFound),
        }
    }
}

impl TaskRepository for InMemoryTaskStore {
    fn get(&self, id: u32) -> Result<Task, RepositoryError> {
        self.get_task(id)
    }

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let tasks = self.lock_tasks()?;
        Ok(tasks.clone())
    }

    fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        let mut tasks = self.lock_tasks()?;
        let id = tasks.len() as u32;
        let task = Task::new(id, new_task.title, new_task.description);
        tasks.push(task);
        Ok(())
    }
    fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        let mut tasks = self.lock_tasks()?;
        let index = tasks
            .iter()
            .position(|t| t.id() == id)
            .ok_or(RepositoryError::NotFound)?;
        tasks.remove(index);
        Ok(())
    }

    fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
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
