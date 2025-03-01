use crate::todo_manager::*;

pub struct InMemoryTodoManager {
    tasks: Vec<Task>,
}

impl InMemoryTodoManager {
    pub fn new() -> Self {
        let tasks = Vec::new();
        Self { tasks }
    }

    fn get(&mut self, title: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.title == title)
    }
}

impl TodoManager for InMemoryTodoManager {
    async fn add(&mut self, title: &str, description: &str, done: bool) {
        let trimmed_title = String::from(title.trim());
        let trimmed_description = String::from(description.trim());

        let task = Task {
            title: trimmed_title,
            description: trimmed_description,
            done,
        };

        self.tasks.push(task);
    }

    async fn remove(&mut self, title: &str) -> bool {
        let task = self.tasks.iter().position(|t| t.title == title);
        match task {
            Some(index) => {
                self.tasks.remove(index);
                true
            }
            None => false,
        }
    }

    async fn edit_title(&mut self, title: &str, new_title: &str) {
        let task = self.get(title);
        if task.is_some() {
            task.unwrap().title = String::from(new_title);
        }
    }

    async fn edit_description(&mut self, str: &str, new_description: &str) {
        let task = self.get(str);
        if task.is_some() {
            task.unwrap().description = String::from(new_description);
        }
    }

    async fn complete(&mut self, title: &str) -> bool {
        let task = self.get(title);
        if task.is_some() {
            task.unwrap().done = true;
            return true;
        }

        false
    }

    async fn print_tasks(&self) {
        self.tasks.iter().for_each(|p| {
            println!("{}, {}, completed: {}", p.title, p.description, p.done);
        });
    }

    async fn is_task_exist(&mut self, title: &str) -> bool {
        match self.get(title) {
            Some(_) => true,
            None => false,
        }
    }
}
