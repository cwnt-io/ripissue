use clap::ValueEnum;
use anyhow::{Result, bail};

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    Todo,
    /// Issue is in execution
    Doing,
}

impl Status {

    pub fn as_str(&self) -> String {
        use Status::*;
        match *self {
            Todo => "todo".to_owned(),
            Doing => "doing".to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        use Status::*;
        match s {
            "todo" => Ok(Todo),
            "doing" => Ok(Doing),
            &_ => bail!("Input \"{}\" is not a valid status", s),
        }
    }

}

// pub trait Status {
//     type Property;
//
//
//
//     pub fn list(elem_id: &str) -> Result<Option<Self>> {
//
//         let mut path = Element::new(issue_id);
//         path.push(&issue_id);
//         path.push(&Status::base_path());
//         let statuses: Vec<String> = WalkDir::new(path)
//             .min_depth(1)
//             .into_iter()
//             .flatten()
//             .map(|e| e.path().to_str().unwrap().to_owned())
//             .collect();
//         match statuses.len() {
//             0 => Ok(None),
//             1 => {
//                 let status_str = statuses.get(0).unwrap();
//                 Ok(Some(Status::from(status_str)?))
//             },
//             _ => bail!("Input \"{}\" doesn't match with any issue", &issue_id),
//         }
//     }
//
// // }
