use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub due_date: Option<String>,
    pub priority: Option<String>,
    pub completed: bool,
}
