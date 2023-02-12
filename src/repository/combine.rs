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

        // If in source directory but not testing, take all files from to_grade

        if is_src && !is_test {
            repo.files.insert(key, value);
        }
    }

    for (key, value) in grading.files {
        // Place all files that don't exist yet -> testing, parent of src
        if !repo.files.contains_key(&key) {
            repo.files.insert(key, value);
        }
    }

    repo
}