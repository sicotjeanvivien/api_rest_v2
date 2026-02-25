pub struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    done: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        Task {
            id,
            title,
            description,
            done: false,
        }
    }

    pub fn id(&self) -> u32 {
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
}
