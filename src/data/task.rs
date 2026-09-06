#[derive(Debug, toasty::Model, Clone)]
#[table = "Tasks"]
pub struct Task {
    #[key]
    #[auto]
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub due_date: Option<jiff::Timestamp>,
}