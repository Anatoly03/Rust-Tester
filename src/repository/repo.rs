use std::{collections::HashMap, fs, io::ErrorKind, path::Path};

/**
   Contains the list of files `[address -> content]`
*/
pub struct Repository {
    pub files: HashMap<String, String>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn from(path: &str) -> Result<Repository, std::io::Error> {
        Self::try_from(Path::new(path))?.clean(path)
    }

    pub fn clean(self, prefix: &str) -> Result<Repository, std::io::Error> {
        let mut clean_repo = Repository::new();

        for (path, content) in self.files {
            let new_path = path.strip_prefix(prefix).unwrap().to_owned();
            clean_repo.files.insert(new_path, content);
        }

        Ok(clean_repo)
    }
}

/**
 * Clones a Repository from Path
 */
impl<'a> TryFrom<&'a Path> for Repository {
    type Error = std::io::Error;

    fn try_from(parent: &'a Path) -> Result<Self, Self::Error> {
        try {
            let mut repo = Repository::new();

            if !fs::metadata(parent)?.is_dir() {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "expected directory, got file",
                ));
            }

            // TODO error handler
            let paths = fs::read_dir(parent)?;

            for path in paths {
                let pt = path.unwrap().path();

                // TODO make this beautiful and readable

                if fs::metadata(&pt)?.is_dir() {
                    let child_repo = Self::try_from(pt.as_path())?;
                    for (path, content) in child_repo.files {
                        repo.files.insert(path, content);
                    }
                } else {
                    repo.files
                        .insert(pt.to_string_lossy().to_string(), fs::read_to_string(pt)?);
                }
            }

            repo
        }
    }
}
