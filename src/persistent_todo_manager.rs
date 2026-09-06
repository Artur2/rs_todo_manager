#![allow(unused_must_use)]

use crate::data::task::Task;
use crate::todo_manager::TodoManager;
use jiff::Span;
use jiff::Zoned;
use std::fs::{exists, remove_file};
use toasty::Db;

pub struct PersistentTodoManager {
    database_name: String,
}

impl PersistentTodoManager {
    pub fn new(database_name: String) -> Self {
        PersistentTodoManager { database_name }
    }

    pub async fn create_database_if_not_exist(&self) -> bool {
        let connection = Db::builder()
            .models(toasty::models!(Task))
            .connect(&self.database_name)
            .await;

        match connection {
            Ok(connection) => {
                connection.push_schema().await;
                true
            }
            Err(e) => false,
        }
    }

    #[allow(dead_code)]
    pub async fn remove_database_if_exist(&self) -> bool {
        let running_dir = std::env::current_dir().unwrap();
        let database_name = running_dir.join(&self.database_name);

        let mut wal_file_name = String::from(&self.database_name.clone());
        wal_file_name.push_str("-wal");
        let wal_name = wal_file_name;

        let mut shm_file_name = String::from(&self.database_name.clone());
        shm_file_name.push_str("-shm");
        let shm_name = shm_file_name;

        if exists(&database_name).is_ok() {
            remove_file(database_name);
        }
        if exists(&wal_name).is_ok() {
            remove_file(wal_name);
        }

        if exists(&shm_name).is_ok() {
            remove_file(shm_name);
        }

        true
    }

    async fn create_connection(&self) -> Db {
        let connection = toasty::Db::builder()
            .models(toasty::models!(Task))
            .connect(&self.database_name)
            .await;

        connection.unwrap_or_else(|_| panic!("cant create database"))
    }
}

impl TodoManager for PersistentTodoManager {
    async fn initialize(&self) {
        let database_creation_result = self.create_database_if_not_exist().await;
        match database_creation_result {
            true => println!("Database successfully created or checked"),
            false => panic!("Database creation failed"),
        }
    }

    async fn add(&mut self, title: &str, description: &str, done: bool) {
        let mut db = self.create_connection().await;

        let result = toasty::create!(Task {
            title: title.to_string(),
            description: description.to_string(),
            done: done
        })
        .exec(&mut db)
        .await;

        match result {
            Ok(_) => println!("task added"),
            Err(_) => panic!("task not added"),
        }
    }

    async fn remove(&mut self, title: String) -> bool {
        let mut db = self.create_connection().await;

        let task = Task::filter(Task::fields().title().eq(title))
            .first()
            .exec(&mut db)
            .await;

        match task {
            Ok(found) => match found {
                None => {
                    println!("Task not found");
                    return false;
                }
                Some(task) => {
                    task.delete().exec(&mut db).await;
                }
            },
            Err(e) => {
                println!("{:?}", e);
                return false;
            }
        }

        true
    }

    async fn edit_title(&mut self, str: &str, new_title: &str) {
        let mut db = self.create_connection().await;

        let entry = Task::filter(Task::fields().title().eq(str))
            .first()
            .exec(&mut db)
            .await;

        match entry {
            Ok(found_task) => match found_task {
                None => {
                    println!("Task not found");
                }
                Some(mut entry) => {
                    toasty::update!(entry { title: new_title })
                        .exec(&mut db)
                        .await;
                }
            },
            Err(_) => {
                println!("Task not found")
            }
        }
    }

    async fn edit_description(&mut self, str: &str, new_description: &str) {
        let mut db = self.create_connection().await;

        let entry = Task::filter(Task::fields().title().eq(str))
            .first()
            .exec(&mut db)
            .await;

        match entry {
            Ok(found_task) => match found_task {
                None => {
                    println!("Task not found");
                }
                Some(mut entry) => {
                    toasty::update!(entry {
                        description: new_description
                    })
                    .exec(&mut db)
                    .await;
                }
            },
            Err(_) => {
                println!("Task not found")
            }
        }
    }

    async fn complete(&mut self, title: String) -> bool {
        let mut db = self.create_connection().await;

        let entry = Task::filter(Task::fields().title().eq(title))
            .first()
            .exec(&mut db)
            .await;

        match entry {
            Ok(found_task) => match found_task {
                None => {
                    println!("Task not found");
                    false
                }
                Some(mut entry) => {
                    toasty::update!(entry { done: true }).exec(&mut db).await;
                    true
                }
            },
            Err(_) => {
                println!("Task not found");
                false
            }
        }
    }

    async fn print_tasks(&self) {
        let mut db = self.create_connection().await;
        let result = toasty::query!(Task).exec(&mut db).await.unwrap();

        for (_, task) in result.iter().enumerate() {
            if task.due_date.is_some() {
                println!(
                    "{}\t{}\t completed: {}, due_date: {}",
                    task.title,
                    task.description,
                    task.done,
                    task.due_date.unwrap().to_string()
                )
            } else {
                println!(
                    "{}\t{}\t completed: {}, due_date: empty",
                    task.title, task.description, task.done
                );
            }
        }
    }

    async fn task_exist(&mut self, title: &str) -> bool {
        let mut db = self.create_connection().await;

        let result = Task::filter(Task::fields().title().eq(title))
            .count()
            .exec(&mut db)
            .await;

        result.is_ok() && result.unwrap() > 0
    }

    async fn set_due_date(&mut self, title: &str, days_count: u64) {
        let mut db = self.create_connection().await;

        let date = Zoned::now()
            .checked_add(Span::new().days(days_count as i64))
            .unwrap();

        let mut task = Task::filter(Task::fields().title().eq(title))
            .first()
            .exec(&mut db)
            .await
            .unwrap()
            .unwrap();

        toasty::update!(task {
            due_date: &date.timestamp()
        })
        .exec(&mut db)
        .await;
    }

    async fn get_existing(&mut self, title: &str) -> Option<Task> {
        let mut db = self.create_connection().await;

        let result = toasty::query!(Task)
            .filter(Task::fields().title().eq(title))
            .first()
            .exec(&mut db)
            .await
            .unwrap();
        result
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn add_task_should_successfully() {
        let random_database_name = "sqlite:".to_string() + Uuid::new_v4().to_string().as_str() + ".db";
        let mut todo_manager = PersistentTodoManager::new(random_database_name.to_string());
        todo_manager.initialize().await;
        todo_manager.add("test", "test description", false).await;

        let task = todo_manager.get_existing("test").await;
        assert_eq!(task.unwrap().title, "test");

        todo_manager.remove_database_if_exist().await;
    }

    #[tokio::test]
    async fn should_complete_task() {
        let random_database_name = "sqlite:".to_string() + Uuid::new_v4().to_string().as_str() + ".db";
        let mut todo_manager = PersistentTodoManager::new(random_database_name.to_string());
        todo_manager.initialize().await;
        todo_manager.add("test", "test description", false).await;
        todo_manager.complete("test".to_string()).await;

        let task = todo_manager.get_existing("test").await;

        assert_eq!(task.unwrap().done, true);

        todo_manager.remove_database_if_exist().await;
    }

    #[tokio::test]
    async fn should_add_due_date() {
        let random_database_name = "sqlite:".to_string() + Uuid::new_v4().to_string().as_str() + ".db";
        let mut todo_manager = PersistentTodoManager::new(random_database_name.to_string());
        todo_manager.initialize().await;
        todo_manager.add("test", "test description", false).await;
        todo_manager.set_due_date("test", 1).await;

        let task = todo_manager.get_existing("test").await;

        assert_eq!(task.unwrap().due_date.is_some(), true);

        todo_manager.remove_database_if_exist().await;
    }
}
