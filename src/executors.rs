use self::{create::Createable, commit::Commitable, close::Closeable, delete::Deleteable};

pub mod create;
pub mod commit;
pub mod close;
pub mod delete;
pub mod run_subc;


pub trait AllExecutables: Createable + Commitable + Closeable + Deleteable {}
impl<T: Createable + Commitable + Closeable + Deleteable> AllExecutables for T {}
