use crate::fs_utils;
use std::env;

#[derive(Debug)]
pub struct InitCommand {
    pub project_name: String,
}
impl InitCommand {
    pub fn new(project_name: String) -> Self {
        InitCommand { project_name }
    }
    pub fn execute_initialization(&self) {
        println!("Initializing {} project", &self.project_name);
        match self.init_project_structure() {
            Ok(_) => println!("Successfully initialized empty project"),
            Err(e) => println!("An Error occurred while initializing the project {e}"),
        }
    }

    /// Initializes the project structure by creating the default directories and files
    ///
    /// # Arguments
    /// * `project_name` - Name of the project to create directories and files for
    ///
    /// # Returns
    /// * `Result<(), &str>` - Returns Ok if all directories and files ar successfully created, or an error if any filesystem operation fails.
    fn init_project_structure(&self) -> Result<(), &str> {
        // get the working directory of the process
        let working_dir = env::current_dir().unwrap();

        // Initialize the default folder structure for the project
        let result = fs_utils::init_default_folders(&self.project_name, &working_dir);
        if result.is_err() {
            return Err("An error occured while initlizing the folder structure of the project");
        }

        // Initialize the default files
        let result = fs_utils::init_default_files(&self.project_name, &working_dir);
        if result.is_err() {
            return Err("An error occured while initializing the default files of the project");
        }

        Ok(())
    }
}
