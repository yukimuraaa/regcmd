use serde::{Deserialize, Serialize};

// Command
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
    pub description: String,
}
