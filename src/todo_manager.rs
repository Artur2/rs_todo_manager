pub struct Task {
    pub title: String,
    pub description: String,
    pub done: bool,
}

pub trait TodoManager {
    /// Adding new task
    async fn add(&mut self, title: &str, description: &str, done: bool);

    /// Removing task from task manager
    async fn remove(&mut self, title: &str) -> bool;

    /// Editing title if we match task by title
    async fn edit_title(&mut self, str: &str, new_title: &str);
    /// Editing description
    async fn edit_description(&mut self, str: &str, new_description: &str);
    /// Completing task
    async fn complete(&mut self, title: &str) -> bool;

    /// Print current tasks
    async fn print_tasks(&self);

    /// Check if tasks completed
    async fn is_task_exist(&mut self, title: &str) -> bool;
}