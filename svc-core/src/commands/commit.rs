// Commit Command Struct
#[derive(Debug)]
pub struct CommitCommand {
    pub message: String,
}

impl CommitCommand {
    pub fn new(message: String) -> Self {
        CommitCommand { message }
    }
}
