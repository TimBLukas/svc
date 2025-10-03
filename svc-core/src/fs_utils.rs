use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Initializes the project structure by creating the default directories and files
///
/// # Arguments
/// * `project_name` - Name of the project to create directories and files for
///
/// # Returns
/// * `Result<(), std::io::Error>` - Returns Ok if all directories and files ar successfully
/// created, or an error if any filesystem operation fails.
pub fn init_project_structure(project_name: &str) -> Result<(), &str> {
    // get the working directory of the process
    let working_dir = env::current_dir().unwrap();

    // Initialize the default folder structure for the project
    let result = init_default_folders(project_name, &working_dir);
    if result.is_err() {
        return Err("An error occured while initlizing the folder structure of the project");
    }

    // Initialize the default files
    let result = init_default_files(project_name, &working_dir);
    if result.is_err() {
        return Err("An error occured while initializing the default files of the project");
    }

    Ok(())
}

/// Creates the default directories for the project.
///
/// # Arguments
/// * `project_name` - the name of the project
/// * `working_dir` - The base working directory where the command has been executed and the project will be created
fn init_default_folders(project_name: &str, working_dir: &PathBuf) -> std::io::Result<()> {
    // Reusable directory builder
    let mut builder = fs::DirBuilder::new();

    // List of diretories to create
    let dirs = [
        project_name.to_string(),
        format!("{project_name}/.svc"),
        format!("{project_name}/.svc/commits"),
        format!("{project_name}/sections"),
        format!("{project_name}/output"),
    ];

    // Iterate over directories to create them recursively
    for dir_name in dirs {
        let full_path = Path::new(working_dir).join(dir_name);
        builder.recursive(true).create(&full_path)?;
    }

    Ok(())
}

/// Creates the default files for the initialization of the project structure
///
/// # Arguments
/// * `project_name` - The name of the project
/// * `working_dir` - The base working_directory where the project will be created
fn init_default_files(project_name: &str, working_dir: &PathBuf) -> std::io::Result<()> {
    // List of files that have to be created for the project initialization
    let files = [
        format!("{}/.svc/config.json", project_name),
        format!("{}/.svc/index.json", project_name),
        format!("{}/sections/01_intro.md", project_name),
    ];

    // iterating over the files and creating them
    for file_name in files {
        let full_path = Path::new(working_dir).join(file_name);
        let _f = fs::File::create(&full_path)?;
    }

    Ok(())
}

/// Creates a new section file inside the "sections" folder (used for add-section command)
///
/// # Arguments
/// * `section_name` - Name of the section (filename without file extension
/// * `working_dir` - Directory in which the command has been executed and the project is located
pub fn init_section_structure(section_name: &str) -> Result<(), &str> {
    // get the working directory of the process
    let working_dir = env::current_dir().unwrap();
    let full_path = working_dir
        .as_path()
        .join(format!("sections/{section_name}.md"));
    println!("{}", full_path.to_string_lossy());
    let f = fs::File::create(full_path);
    if f.is_err() {
        return Err("Error while attempting to create the files for the new Section");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_project_structure_creates_dirs_and_files() {
        // Create temporary files and directories
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test_project";

        // Call the function with temp_dir as working Directory
        let _ = init_default_folders(project_name, &temp_dir.path().to_path_buf());
        let _ = init_default_files(project_name, &temp_dir.path().to_path_buf());

        // Check directories
        assert!(temp_dir.path().join(project_name).exists());
        assert!(
            temp_dir
                .path()
                .join(format!("{project_name}/.svc"))
                .is_dir()
        );
        assert!(
            temp_dir
                .path()
                .join(format!("{project_name}/sections"))
                .is_dir()
        );

        // Check files
        assert!(
            temp_dir
                .path()
                .join(format!("{project_name}/.svc/config.json"))
                .is_file()
        );
        assert!(
            temp_dir
                .path()
                .join(format!("{project_name}/sections/01_intro.md"))
                .is_file()
        );
    }

    #[test]
    fn folder_with_same_name_as_project_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        // Change working directory to temp_dir
        env::set_current_dir(temp_dir.path()).unwrap();
        let project_name = "test_project";
        // Create a directory with the same name
        let _ = fs::create_dir(temp_dir.path().join(project_name));
        // Try to initialize a new project with the same name
        match init_project_structure(project_name) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
