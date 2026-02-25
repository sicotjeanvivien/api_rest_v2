use crate::{
    domain::task::model::{NewTask, Task, UpdateTask},
    errors::repository_error::RepositoryError,
};

pub trait TaskRepository {
    fn get(&self, id: u32) -> Result<Task, RepositoryError>;
    fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;
    fn create(&self, new_task: NewTask) -> Result<(), RepositoryError>;
    fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError>;
    fn delete(&self, id: u32) -> Result<(), RepositoryError>;
}
