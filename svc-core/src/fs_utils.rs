use std::env;
use std::fs;

pub fn init_project_structure(project_name: &str) -> Result<(), std::io::Error> {
    let working_dir_path = env::current_dir()?;
    let working_dir = working_dir_path
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid UTF-8 path"))?;
    init_folders(project_name, working_dir)?;
    init_files(project_name, working_dir)?;
    Ok(())
}

fn init_folders(project_name: &str, working_dir: &str) -> std::io::Result<()> {
    let mut builder = fs::DirBuilder::new();

    // Directories to create for project initialization
    let dirs = [
        project_name.to_string(),
        format!("{project_name}/.svc"),
        format!("{project_name}/.svc/commits"),
        format!("{project_name}/sections"),
        format!("{project_name}/output"),
    ];

    dirs.iter().for_each(|dir| {
        let full_path = format!("{working_dir}/{dir}");
        println!("Creating {full_path}");
        builder.recursive(true).create(full_path).unwrap()
    });

    Ok(())
}

fn init_files(project_name: &str, working_dir: &str) -> std::io::Result<()> {
    // Files to create for project initialization
    let files = [
        format!("{}/.svc/config.json", project_name),
        format!("{}/.svc/index.json", project_name),
        format!("{}/sections/01_intro.md", project_name),
    ];

    files.iter().for_each(|file| {
        let full_path = format!("{working_dir}/{file}");
        println!("Creating {full_path}");

        let _f = fs::File::create(full_path).unwrap();
    });
    Ok(())
}

#[cfg(test)]
mod test {}
