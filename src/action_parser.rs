use crate::todo_actions::TodoAction;

pub trait ActionParser {
    fn parse(&self, input: &str) -> TodoAction;
}

pub struct StringActionParser;

impl StringActionParser {
    pub fn new() -> StringActionParser {
        StringActionParser {}
    }
}

impl ActionParser for StringActionParser {
    fn parse(&self, input: &str) -> TodoAction {
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
}
