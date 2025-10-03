// Build Struct
#[derive(Debug)]
pub struct BuildCommand {
    pub name: String,
}

impl BuildCommand {
    pub fn new(name: String) -> Self {
        BuildCommand { name }
    }
}
