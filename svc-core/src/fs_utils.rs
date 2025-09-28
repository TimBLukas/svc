use std::fs;

pub fn init_project_structure() {
    let mut builder = fs::DirBuilder::new();
    let paths = ["", ""];

    paths
        .iter()
        .for_each(|path| builder.recursive(true).create(path).unwrap());

    assert!(fs::metadata(paths[0]).unwrap().is_dir());
}
