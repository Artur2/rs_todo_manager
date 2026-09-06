mod action_dispatcher;
mod action_parser;
mod in_memory_todo_manager;
mod persistent_todo_manager;
mod todo_actions;
mod todo_manager;
pub mod data;

use crate::action_dispatcher::{ActionDispatcher, DefaultActionDispatcher};
use crate::action_parser::{ActionParser, StringActionParser};
use crate::in_memory_todo_manager::InMemoryTodoManager;
use crate::persistent_todo_manager::PersistentTodoManager;
use clap::Parser;
use std::io::*;
use todo_manager::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Use in memory todo manager
    #[arg(short, long, default_value = "false")]
    pub in_memory: bool,
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    let todo_manager = create_manager(cli.in_memory);
    todo_manager.initialize().await;
    let parser = create_action_parser();
    let mut dispatcher = create_dispatcher(parser, todo_manager);
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

        let result = dispatcher.dispatch(&mut line).await;
        if result {
            break;
        }
    }
}

pub fn create_manager(in_memory: bool) -> impl TodoManager {
    if in_memory {
        InMemoryTodoManager::new();
    }

    let database_name = String::from("todo_manager.sqlite");
    PersistentTodoManager::new(database_name)
}

pub fn create_action_parser() -> impl ActionParser {
    StringActionParser::new()
}

pub fn create_dispatcher<P, M>(parser: P, todo_manager: M) -> impl ActionDispatcher<P, M>
where
    P: ActionParser,
    M: TodoManager,
{
    DefaultActionDispatcher::new(parser, todo_manager)
}
