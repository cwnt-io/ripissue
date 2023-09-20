use clap::Args;

use crate::properties::repos::Repos;

use super::elem::Elem;

#[derive(Debug, Clone)]
pub struct Project {
    elem: Elem,
    repos: Repos,
}

impl Project {
    pub fn raw() -> Self {
        let elem = Elem::raw("Project");
        Self { elem, repos: Repos::default() }
    }
}

#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[arg(short, long)]
    repos: Option<Vec<String>>,
}
