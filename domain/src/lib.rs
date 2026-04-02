pub mod error;
pub mod task;
pub mod user;

pub use task::NewTask;
pub use task::Task;
pub use task::TaskRepository;
pub use task::UpdateTask;

pub use user::NewUser;
pub use user::User;
pub use user::UserAuth;
pub use user::UserRegister;
pub use user::UserRepository;

pub use error::RepositoryError;

#[cfg(test)]
mod tests {
    use derive_macros::Builder;

    #[derive(Builder)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_builder() {
        let point = Point::builder().x(10).y(20).build();

        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }
}
