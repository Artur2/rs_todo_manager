use crate::action_parser::ActionParser;
use crate::todo_actions::TodoAction;
use crate::todo_manager::TodoManager;
use chrono::{DateTime, Local, Utc};
use std::cell::RefCell;
use std::io::stdin;

pub trait ActionDispatcher<P, M>
where
    P: ActionParser,
    M: TodoManager,
{
    // returns true if exit
    async fn dispatch(&mut self, input: &str) -> bool;
}

pub struct DefaultActionDispatcher<P, M>
where
    P: ActionParser,
    M: TodoManager,
{
    pub action_parser: RefCell<P>,
    pub todo_manager: RefCell<M>,
}

impl<P, M> DefaultActionDispatcher<P, M>
where
    P: ActionParser,
    M: TodoManager,
{
    pub fn new(action_parser: P, todo_manager: M) -> DefaultActionDispatcher<P, M> {
        DefaultActionDispatcher {
            action_parser: RefCell::new(action_parser),
            todo_manager: RefCell::new(todo_manager),
        }
    }

    fn parse_action(&self, input: &str) -> TodoAction {
        let parserCell = &self.action_parser;
        let parser = parserCell.borrow();
        parser.parse(input)
    }

    async fn add_task(&mut self) {
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

        let mut todo_instance = self.todo_manager.borrow_mut();

        if todo_instance.is_task_exist(&title).await {
            println!("Task {} already exists", title);
            return;
        }

        todo_instance.add(&title, &description, false).await;
    }

    async fn remove_task(&mut self) {
        println!("specify title to remove task");
        let mut title = String::default();

        stdin().read_line(&mut title).unwrap();
        title = title.trim().to_string();

        if title.is_empty() {
            println!("Title is not specified");
        }

        let mut todo_instance = self.todo_manager.borrow_mut();
        match todo_instance.remove(title.to_owned()).await {
            true => {
                println!("task with title {} removed", title);
            }
            false => {
                println!("task with title {} not found", title);
            }
        }
    }

    async fn complete_task(&mut self) {
        let mut title = String::default();
        println!("Specify title for task to complete");
        stdin().read_line(&mut title).unwrap();
        title = title.trim().to_string();
        if title.is_empty() {
            println!("Title is not specified");
        }

        let mut todo_instance = self.todo_manager.borrow_mut();
        match todo_instance.complete(title.to_owned()).await {
            true => println!("task {} completed", title),
            false => println!("task {} not completed", title),
        }
    }

    async fn edit_task_title(&mut self) {
        let mut title = String::default();
        let mut new_title = String::default();

        println!("Specify title for search task:");
        stdin().read_line(&mut title).unwrap();

        title = title.trim().to_string();

        if title.is_empty() {
            println!("Title is not specified");
        }

        let mut todo_instance = self.todo_manager.borrow_mut();
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

    async fn edit_task_description(&mut self) {
        let mut title = String::default();
        let mut new_description = String::default();

        println!("Specify title for search task:");
        stdin().read_line(&mut title).unwrap();

        title = title.trim().to_string();

        if title.is_empty() {
            println!("Title is not specified");
        }

        let mut todo_instance = self.todo_manager.borrow_mut();
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

    async fn set_due_date(&mut self) {
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

        let mut todo_instance = self.todo_manager.borrow_mut();
        todo_instance.set_due_date(&title, days.unwrap()).await;
    }
}

impl<P, M> ActionDispatcher<P, M> for DefaultActionDispatcher<P, M>
where
    P: ActionParser,
    M: TodoManager,
{
    async fn dispatch(&mut self, input: &str) -> bool {
        let result = self.parse_action(input);
        let start = Local::now();
        let mut return_result = false;
        match result {
            TodoAction::ADD => {
                self.add_task().await;
            }
            TodoAction::REMOVE => {
                self.remove_task().await;
            }
            TodoAction::EDITTITLE => {
                self.edit_task_title().await;
            }
            TodoAction::SETDUEDATE => {
                self.set_due_date().await;
            }
            TodoAction::EDITDESCRIPTION => {
                self.edit_task_description().await;
            }
            TodoAction::COMPLETE => {
                self.complete_task().await;
            }
            TodoAction::LIST => {
                let todo_manager = self.todo_manager.borrow();
                todo_manager.print_tasks().await;
            }
            TodoAction::QUIT => {
                return_result = true;
            }
            _ => {
                println!("Invalid command");
            }
        }

        let diff = Local::now() - start;
        println!("elapsed time: {}", diff.num_microseconds().unwrap());
        return_result
    }
}
