mod in_memory_todo_manager;
mod persistent_todo_manager;
mod todo_actions;
mod todo_manager;

use crate::persistent_todo_manager::PersistentTodoManager;
use std::io::*;
use todo_actions::TodoAction;
use todo_manager::*;

#[tokio::main]
pub async fn main() {
    let database_name = String::from("todo_manager.sqlite");
    let mut persistent_todo_manager = PersistentTodoManager::new(database_name);
    persistent_todo_manager.initialize().await;

    println!("Welcome to todo manager");
    println!(
        r#"Please enter a,e,l,q,c
                   a: for add
                   et: for edit title
                   ed: for edit description
                   dd: for set due_date
                   l: for list
                   c: for complete
                   q: for quit"#
    );

    loop {
        let mut line = String::default();
        stdin().read_line(&mut line).unwrap();
        let mut exit = false;

        match detect_action(&line) {
            TodoAction::LIST => persistent_todo_manager.print_tasks().await,
            TodoAction::ADD => add_task(&mut persistent_todo_manager).await,
            TodoAction::REMOVE => remove_task(&mut persistent_todo_manager).await,
            TodoAction::EDITTITLE => edit_task_title(&mut persistent_todo_manager).await,
            TodoAction::SETDUEDATE => set_due_date(&mut persistent_todo_manager).await,
            TodoAction::EDITDESCRIPTION => {
                edit_task_description(&mut persistent_todo_manager).await
            }
            TodoAction::COMPLETE => complete_task(&mut persistent_todo_manager).await,
            TodoAction::QUIT => exit = true,
            _ => println!("Invalid command"),
        }

        if exit {
            break;
        }

        println!("Activity is complete, specify next");
    }
}

pub fn detect_action(input: &str) -> TodoAction {
    match input.trim() {
        "q" => TodoAction::QUIT,
        "a" => TodoAction::ADD,
        "et" => TodoAction::EDITTITLE,
        "ed" => TodoAction::EDITDESCRIPTION,
        "l" => TodoAction::LIST,
        "c" => TodoAction::COMPLETE,
        "r" => TodoAction::REMOVE,
        "dd" => TodoAction::SETDUEDATE,
        _ => TodoAction::NONE,
    }
}

pub async fn add_task(todo_instance: &mut PersistentTodoManager) {
    let mut title = String::default();
    let mut description = String::default();

    println!("Specify title:");
    stdin().read_line(&mut title).unwrap();
    title = title.trim().to_string();

    if title.is_empty() {
        println!("Please enter a title, provided is empty");
        return;
    }

    println!("Specify description:");
    stdin().read_line(&mut description).unwrap();
    description = description.trim().to_string();

    if description.is_empty() {
        println!("Please enter a description, provided is empty");
        return;
    }

    if todo_instance.is_task_exist(&title).await {
        println!("Task {} already exists", title);
        return;
    }

    todo_instance.add(&title, &description, false).await;
}

pub async fn remove_task(todo_instance: &mut PersistentTodoManager) {
    println!("specify title to remove task");
    let mut title = String::default();

    stdin().read_line(&mut title).unwrap();
    title = title.trim().to_string();

    if title.is_empty() {
        println!("Title is not specified");
    }

    match todo_instance.remove(&title).await {
        true => {
            println!("task with title {} removed", title);
        }
        false => {
            println!("task with title {} not found", title);
        }
    }
}

pub async fn complete_task(todo_instance: &mut PersistentTodoManager) {
    let mut title = String::default();
    println!("Specify title for task to complete");
    stdin().read_line(&mut title).unwrap();
    title = title.trim().to_string();
    if title.is_empty() {
        println!("Title is not specified");
    }

    match todo_instance.complete(&title).await {
        true => println!("task {} completed", title),
        false => println!("task {} not completed", title),
    }
}

pub async fn edit_task_title(todo_instance: &mut PersistentTodoManager) {
    let mut title = String::default();
    let mut new_title = String::default();

    println!("Specify title for search task:");
    stdin().read_line(&mut title).unwrap();

    title = title.trim().to_string();

    if title.is_empty() {
        println!("Title is not specified");
    }

    if !todo_instance.is_task_exist(&title).await {
        println!("Task {} not found", title);
        return;
    }

    println!("Specify new title:");
    stdin().read_line(&mut new_title).unwrap();
    new_title = new_title.trim().to_string();
    if new_title.is_empty() {
        println!("New title is not specified");
        return;
    }
    todo_instance.edit_title(&title, &new_title).await;
}

pub async fn edit_task_description(todo_instance: &mut PersistentTodoManager) {
    let mut title = String::default();
    let mut new_description = String::default();

    println!("Specify title for search task:");
    stdin().read_line(&mut title).unwrap();

    title = title.trim().to_string();

    if title.is_empty() {
        println!("Title is not specified");
    }

    if !todo_instance.is_task_exist(&title).await {
        println!("Task {} not found", title);
        return;
    }

    println!("Specify new description:");
    stdin().read_line(&mut new_description).unwrap();
    new_description = new_description.trim().to_string();
    if new_description.is_empty() {
        println!("New title is not specified");
        return;
    }
    todo_instance
        .edit_description(&title, &new_description)
        .await;
}

pub async fn set_due_date(todo_instance: &mut PersistentTodoManager) {
    let mut days_as_string = String::default();
    let mut title_raw = String::default();

    println!("Specify title for task:");
    stdin().read_line(&mut title_raw).unwrap();
    let title = title_raw.trim().to_string();

    stdin().read_line(&mut days_as_string).unwrap();

    let days_as_string = days_as_string.trim().to_string();
    let days = days_as_string.parse::<u64>();
    if days.is_err() {
        println!("Invalid days value");
        return;
    }

    todo_instance.set_due_date(&title, days.unwrap()).await;
}
