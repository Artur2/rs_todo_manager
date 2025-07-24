use chrono::{DateTime, Utc};

#[derive(Default, Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub done: bool,
    pub due_date: Option<DateTime<Utc>>,
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            title: self.title.clone(),
            description: self.description.clone(),
            done: self.done,
            due_date: self.due_date.clone(),
        }
    }
}

pub trait TodoManager {
    async fn initialize(&self);

    /// Adding new task
    async fn add(&mut self, title: &str, description: &str, done: bool);

    /// Removing task from task manager
    async fn remove(&mut self, title: String) -> bool;

    /// Editing title if we match task by title
    async fn edit_title(&mut self, str: &str, new_title: &str);
    /// Editing description
    async fn edit_description(&mut self, str: &str, new_description: &str);
    /// Completing task
    async fn complete(&mut self, title: String) -> bool;

    /// Print current tasks
    async fn print_tasks(&self);

    /// Check if tasks completed
    async fn is_task_exist(&mut self, title: &str) -> bool;

    /// Adding due date to task
    async fn set_due_date(&mut self, title: &str, days_count: u64);

    async fn get_existing(&mut self, title: &str) -> Option<Task>;
}

mod tests {
    #[allow(unused_imports)]
    use crate::{InMemoryTodoManager, TodoManager};

    #[tokio::test]
    pub async fn should_create_task() {
        let mut todo_manager = InMemoryTodoManager::new();
        todo_manager.add("test", "t", false).await;
        let task_exist = todo_manager.is_task_exist("test").await;
        assert_eq!(true, task_exist);
    }
    #[tokio::test]
    pub async fn should_not_exist_task() {
        let mut todo_manager = InMemoryTodoManager::new();
        todo_manager.add("test", "t", false).await;
        let task_exist = todo_manager.is_task_exist("t").await;
        assert_eq!(false, task_exist);
    }

    #[tokio::test]
    pub async fn should_complete_task() {
        let mut todo_manager = InMemoryTodoManager::new();
        todo_manager.add("test", "t", false).await;
        todo_manager.complete(String::from("test")).await;
        todo_manager.print_tasks().await;

        let task = todo_manager.get_existing("test").await;
        assert_eq!(task.is_some(), true);
    }

    #[tokio::test]
    pub async fn should_add_due_date() {
        let mut todo_manager = InMemoryTodoManager::new();
        todo_manager.add("test", "t", false).await;
        todo_manager.set_due_date("test", 1).await;

        let task = todo_manager.get_existing("test").await;
        let due_date = task.unwrap().due_date;

        assert_eq!(due_date.is_some(), true);
    }
}
