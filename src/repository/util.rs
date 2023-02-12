use std::{fs, path::Path};

use super::repo::Repository;

/**
 * Combines a grading and to-grade repository into one rust repository
 */
pub fn combine(grading: Repository, to_grade: Repository) -> Repository {
    let mut repo = Repository::new();

    // TODO don't hardcore this

    for (key, value) in to_grade.files {
        let is_src = (&key).starts_with("src/");
        let is_test = (&key).starts_with("src/test/");
        let is_main = (&key).starts_with("src/main.rs");

        // If in source directory but not testing, take all files from to_grade

        // println!("K: {},  B: {}", &key, is_src && !is_test && !is_main);
        if is_src && !is_test && !is_main {
            repo.files.insert(key, value);
        }
    }

    for (key, value) in grading.files {
        // Place all files that don't exist yet -> testing, parent of src
        // println!("K: {},   C: {}", &key, !repo.files.contains_key(&key));
        if !repo.files.contains_key(&key) {
            repo.files.insert(key, value);
        }
    }

    repo
}

impl Repository {
    pub fn write_to(&self, path: &str) -> Result<(), std::io::Error> {
        // println!("{:#?}", self.files);
        for (key, value) in &self.files {
            let p = path.to_string() + "/" + key;
            // println!("{}", p);
            // https://stackoverflow.com/a/59046435
            let prefix = Path::new(&p).parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            fs::write(p, value)?;
        }

        Ok(())
    }
}
