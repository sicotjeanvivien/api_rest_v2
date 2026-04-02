use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    done: bool,
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

    #[allow(dead_code)]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    #[allow(dead_code)]
    pub fn set_description(&mut self, description: Option<String>) -> &Self {
        self.description = description;
        self
    }

    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    proptest! {

    #[test]
    fn title_est_toujours_preserve(title in ".*") {
           let task = Task::new(1, title.clone(), None, false);
           assert_eq!(task.title(), title);
       }

    #[test]
      fn done_est_toujours_preserve(done: bool){
          let task = Task::new(1, "title".to_string(), None, done.clone());
           assert_eq!(task.done(), done);
      }
    }
}
