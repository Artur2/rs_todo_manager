#![allow(unused_must_use)]

use crate::todo_manager::TodoManager;
use chrono::{DateTime, Days, Utc};
use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};
use std::ops::Add;

pub struct PersistentTodoManager {
    pub database_name: String,
}

impl PersistentTodoManager {
    pub fn new(database_name: String) -> Self {
        PersistentTodoManager { database_name }
    }

    pub async fn initialize(&self) {
        let database_creation_result = self.create_database_if_not_exist().await;
        match database_creation_result {
            true => println!("Database successfully created or checked"),
            false => panic!("Database creation failed"),
        }

        let migration_result = self.migrate().await;
        match migration_result {
            true => println!("Database migrated successfully"),
            false => panic!("Database migration failed"),
        }
    }

    pub async fn create_database_if_not_exist(&self) -> bool {
        let result = Sqlite::database_exists(&self.database_name).await.unwrap();
        if result == false {
            println!("Creating database {}", &self.database_name);
            match Sqlite::create_database(&self.database_name).await {
                Ok(_) => {
                    println!("Migration succeeded");
                    true
                }
                Err(error) => {
                    println!("error: {}", error);
                    false
                }
            }
        } else {
            println!("Database already exists");
            true
        }
    }

    pub async fn migrate(&self) -> bool {
        let running_dir = std::env::current_dir().unwrap();
        let migrations = std::path::Path::new(&running_dir).join("./migrations");

        let migration_folder_exists = migrations.try_exists();

        if migration_folder_exists.is_err() {
            panic!("Could not find migration folder");
        }

        if !migration_folder_exists.unwrap() {
            panic!("Migrations folder does not exist");
        }

        let db = self.create_connection().await;
        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&db)
            .await;

        match migration_results {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn create_connection(&self) -> Pool<Sqlite> {
        let db = SqlitePool::connect(&self.database_name).await;
        if db.is_err() {
            panic!("Could not connect to database");
        }
        db.unwrap()
    }
}

impl TodoManager for PersistentTodoManager {
    async fn add(&mut self, title: &str, description: &str, done: bool) {
        let db = self.create_connection().await;
        let done_numeric: i32;
        match done {
            true => done_numeric = 1,
            false => done_numeric = 0,
        }

        let result =
            sqlx::query("INSERT INTO Tasks(title, description, completed) VALUES (?, ?, 0)")
                .bind(&title)
                .bind(&description)
                .bind(&done_numeric)
                .execute(&db)
                .await;

        match result {
            Ok(_) => println!("task added"),
            Err(_) => panic!("task not added"),
        }
    }

    async fn remove(&mut self, title: &str) -> bool {
        let db = self.create_connection().await;

        sqlx::query("DELETE FROM Tasks WHERE title = ?")
            .bind(&title)
            .execute(&db)
            .await
            .expect("Not Deleted");
        true
    }

    async fn edit_title(&mut self, str: &str, new_title: &str) {
        let db = self.create_connection().await;

        sqlx::query("UPDATE Tasks SET 'title' = ? WHERE title = ?")
            .bind(&new_title)
            .bind(&str)
            .execute(&db)
            .await
            .expect("Not Deleted");
    }

    async fn edit_description(&mut self, str: &str, new_description: &str) {
        let db = self.create_connection().await;

        sqlx::query("UPDATE Tasks SET 'description' = ? WHERE title = ?")
            .bind(&new_description)
            .bind(&str)
            .execute(&db)
            .await
            .expect("Not updated");
    }

    async fn complete(&mut self, title: &str) -> bool {
        let db = self.create_connection().await;

        let result = sqlx::query("UPDATE Tasks SET 'completed' = ? WHERE title = ?")
            .bind(1)
            .bind(&title)
            .execute(&db)
            .await;

        result.is_ok()
    }

    async fn print_tasks(&self) {
        let db = self.create_connection().await;
        let result = sqlx::query("SELECT * FROM Tasks")
            .fetch_all(&db)
            .await
            .unwrap();

        for (_, task) in result.iter().enumerate() {
            let title = task.get::<String, &str>("title");
            let description = task.get::<String, &str>("description");
            let done = task.get::<u8, &str>("completed");
            let due_date = task.get::<i64, &str>("due_date");

            if due_date > 0 {
                let due_date = DateTime::from_timestamp(due_date, 0);
                if due_date.is_some() {
                    println!(
                        "{}\t{}\t completed: {}, due_date: {}",
                        title,
                        description,
                        done,
                        due_date.unwrap().to_string()
                    );
                }
            } else {
                println!(
                    "{}\t{}\t completed: {}, due_date: empty",
                    title, description, done
                );
            }
        }
    }

    async fn is_task_exist(&mut self, title: &str) -> bool {
        let db = self.create_connection().await;

        let result = sqlx::query("SELECT * FROM Tasks WHERE title = ?")
            .bind(&title)
            .fetch_all(&db)
            .await
            .unwrap();

        result.len() != 0
    }

    async fn set_due_date(&mut self, title: &str, days_count: u64) {
        let db = self.create_connection().await;

        let date = Utc::now().add(Days::new(days_count));
        let timestamp = date.timestamp();

         sqlx::query("UPDATE Tasks SET 'due_date' = ? WHERE title = ?")
            .bind(timestamp)
            .bind(&title)
            .execute(&db)
            .await;
    }
}
