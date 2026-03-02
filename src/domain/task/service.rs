use crate::{
    domain::task::{
        model::{NewTask, Task, UpdateTask},
        repository::TaskRepository,
    },
    errors::repository_error::RepositoryError,
};

pub struct TaskService {
    repository: Box<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(repository: Box<dyn TaskRepository>) -> Self {
        TaskService { repository }
    }

    pub fn get(&self, id: u32) -> Result<Task, RepositoryError> {
        self.repository.get(id)
    }

    pub fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        self.repository.get_all()
    }

    pub fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        self.repository.create(new_task)
    }

    pub fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        self.repository.update(update_task)
    }

    pub fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        self.repository.delete(id)
    }
}
