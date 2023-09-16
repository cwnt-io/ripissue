use anyhow::{Result, bail};

use super::elem::{Elem, ElemBase};


pub struct Elems<TC>(TC);

impl<TC: ElemsBase> Elems<TC> {
    pub fn new(elems: TC) -> Self {
        Self(elems)
    }
    pub fn es(&mut self) -> &mut TC {
        &mut self.0
    }
}

pub trait ElemsBase {
    fn new() -> Self;
    // fn add<TI>(&mut self, elem: Elem<TI>) -> Result<()>;
}




// #[derive(Debug)]
// pub struct Elems(BTreeMap<String, Elem>);
//
// impl Elems {
//     fn new() -> Self {
//         Elems(BTreeMap::new())
//     }
//     fn add(&mut self, item: Elem) -> Result<()> {
//         let id = item.id().to_owned();
//         let etype_str = item.etype().as_ref().to_owned();
//         if self.0.insert(id.clone(),item).is_some() {
//             bail!("{} #{} already exists.", etype_str, id);
//         }
//         Ok(())
//     }
//     // fn update_all(&mut self) -> Result<()> {
//     //     let mut all_elems = vec![];
//     //     for p in Self::Item::base_path_all().iter() {
//     //         let vec_elems = traverse_dirs(p);
//     //         all_elems.extend(vec_elems);
//     //     }
//     //     for e in all_elems.iter() {
//     //         let e_str = e.to_str().unwrap();
//     //         let mut issue = Self::Item::raw(e_str)?;
//     //         let vec_tags = Tag::vec_tags_from_files(&issue.tags_path());
//     //         issue.set_tags(vec_tags);
//     //         let status = Status::status_from_files(&issue.status_path())?;
//     //         issue.set_status(status);
//     //         // let elem = get_elem_from_path(elem_raw)?;
//     //         self.add(issue)?;
//     //     }
//     //     Ok(())
//     // }
// }
