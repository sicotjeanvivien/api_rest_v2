use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    done: bool,
}

impl Task {
    pub(crate) fn new(id: i32, title: String, description: Option<String>, done: bool) -> Self {
        Task {
            id,
            title,
            description,
            done,
        }
    }

    pub(crate) fn id(&self) -> i32 {
        self.id
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub(crate) fn done(&self) -> bool {
        self.done
    }

    #[allow(dead_code)]
    pub(crate) fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn set_description(&mut self, description: Option<String>) -> &Self {
        self.description = description;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn set_done(&mut self, done: bool) -> &Self {
        self.done = done;
        self
    }
}

#[derive(Deserialize)]
pub(crate) struct NewTask {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct UpdateTask {
    pub(crate) id: i32,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) done: Option<bool>,
}
