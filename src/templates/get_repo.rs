use crate::repository::repo::*;

impl Repository {
    pub fn get_from_template(key: &str, name: &str) -> Result<Repository, std::io::Error> {
        let mut repo = Repository::new();
        let template_repo = Repository::from(&format!("src/templates/{}", key))?;

        for (key, value) in template_repo.files {
            let new_key = str::replace(&key, "NAME", name);
            let new_content = str::replace(&value, "NAME", name);
            repo.files.insert(new_key, new_content);
        }

        Ok(repo)
    }
}