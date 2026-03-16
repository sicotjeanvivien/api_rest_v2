use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

impl Task {
    pub fn new(id: i32, title: String, description: Option<String>, done: bool) -> Self {
        Task {
            id,
            title,
            description,
            done,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    pub fn set_description(&mut self, description: Option<String>) -> &Self {
        self.description = description;
        self
    }

    pub fn set_done(&mut self, done: bool) -> &Self {
        self.done = done;
        self
    }
}

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}
