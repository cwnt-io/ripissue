use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Repo(PathBuf);

#[derive(Debug, Clone)]
pub struct Repos(Vec<Repo>);
