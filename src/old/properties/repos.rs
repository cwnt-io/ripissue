use std::{path::PathBuf, str::FromStr, slice::Iter};

#[derive(Debug, Clone)]
pub struct Repo(PathBuf);

impl Repo {
    pub fn from_str(s: &str) -> Self {
        let mut repo_path = PathBuf::from_str("../").unwrap();
        repo_path.push(s);
        Self(repo_path)
    }
    pub fn get(&self) -> &PathBuf {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Repos(Vec<Repo>);

impl Repos {
    pub fn from_vec_str(v: Vec<String>) -> Self {
        Self(v.iter().map(|s| Repo::from_str(&s)).collect())
    }
    pub fn iter(&self) -> Iter<'_, Repo> {
        self.0.iter()
    }
}

impl Default for Repos {
    fn default() -> Self {
        Self(Vec::default())
    }
}

