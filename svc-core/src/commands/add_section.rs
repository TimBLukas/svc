use crate::fs_utils;

// Struct of the add section command
#[derive(Debug)]
pub struct AddSectionCommand {
    pub section_name: String,
}
impl AddSectionCommand {
    pub fn new(section_name: String) -> Self {
        AddSectionCommand { section_name }
    }
    pub fn execute_add_section(&self) {
        println!("Adding new section ({})", self.section_name);
        match fs_utils::init_section_structure(&self.section_name) {
            Ok(_) => println!("Successfully initialized new section"),
            Err(e) => println!("Unable to create section: {e}"),
        }
    }
}
