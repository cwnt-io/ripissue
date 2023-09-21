// TODO: implement a new type that is a wrapper around Elem
// For Elem: create a new field that saves repo_from
// -- saves it when elem is read/created

use super::{elem::Elem, elem_type::ElemType};

#[derive(Debug, Clone)]
pub struct Sprint {
    sprint: Elem,
    issues: Vec<Elem>,
}
