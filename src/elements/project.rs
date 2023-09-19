use super::{elem::Elem, repo::Repos};

#[derive(Debug, Clone)]
pub struct Project {
    elem: Elem,
    repos: Repos,
}
