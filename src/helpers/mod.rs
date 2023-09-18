use std::{
    path::{PathBuf, Path},
    str::FromStr,
    fs::{File, create_dir_all},
    io::{Write, BufWriter, Stdout, stdout}, iter::Flatten,
};

use slugify::slugify;
use anyhow::{Context, Result, bail};
use git2::{Repository, IndexAddOption};
use walkdir::WalkDir;

// pub fn type_to_str<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

pub fn wstdout() -> BufWriter<Stdout> {
    let stdout = stdout();
    BufWriter::new(stdout)
}

pub fn walkdir_into_iter(path: &PathBuf) -> Flatten<walkdir::IntoIter> {
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .flatten()
}

pub fn traverse_files(path: &PathBuf) -> Vec<PathBuf> {
    let walk_iter = walkdir_into_iter(path);
    walk_iter
        .map(|e| e.into_path())
        .collect()
}

// pub fn traverse_dirs(path: &PathBuf) -> Vec<PathBuf> {
//     let walk_iter = walkdir_into_iter(path);
//     walk_iter
//         .filter(|e| e.file_type().is_dir())
//         .map(|e| e.into_path())
//         .collect()
// }

// pub fn get_all_elems<T>() -> Result<BTreeMap<String, impl Element>>
//     where T: Element,
//           <T as Element>::Item: Element,
// {
//     let mut all_elems = vec![];
//     for p in T::base_path_all().iter() {
//         let vec_elems = traverse_dirs(p);
//         all_elems.extend(vec_elems);
//     }
//     let mut map = BTreeMap::new();
//     for e in all_elems.iter() {
//         let e_str = e.to_str().unwrap();
//         let elem_raw = T::raw(e_str)?;
//         let elem = get_elem_from_path(elem_raw)?;
//         map.insert(elem.id(), elem);
//     }
//     Ok(map)
// }

pub fn sys_base_path() -> PathBuf {
    PathBuf::from_str("ripi").unwrap()
}

pub fn get_closed_dir() -> PathBuf {
    let mut closed = sys_base_path();
    closed.push(".closed");
    closed
}

pub fn write_file(dir: &PathBuf, file: &str, content: Option<&str>) -> Result<()> {
    create_dir_all(dir)
        .with_context(|| format!("Could not create {}",
                                 dir.display()))?;
    let mut file_path = dir.clone();
    file_path.push(file);
    let mut file = File::create(&file_path)
        .with_context(|| format!("Could not create file {}",
                                 &file_path.display()))?;
    if let Some(c) = content {
        file.write_all(c.as_bytes())
            .with_context(|| format!(
                    "Could not write content to file {}",
                    file_path.display()))?;
    }
    Ok(())
}

pub fn is_not_empty(arg: &str) -> Result<String> {
    if arg.is_empty() {
        bail!("issue create: name cannot be empty");
    }
    Ok(arg.to_string())
}

pub fn slug(s: &str) -> String {
    slugify!(&s.to_lowercase(), separator = "_")
}

pub fn slug_tag(s: &str) -> String {
    slugify!(&s.to_lowercase(), separator = "-")
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_owned()
}

// pub fn get_parent_dir(path: &PathBuf) -> String {
//     path.parent().unwrap().to_str().unwrap().to_owned()
// }

pub fn git_commit(files_to_add: Option<&[String]>, msg: &str) -> Result<()> {
    let repo = Repository::open(".")
        .with_context(|| "failed to open repository")?;
    let signature = repo.signature()?;
    let mut index = repo.index()?;
    if let Some(files_to_add) = files_to_add {
        index.add_all(files_to_add.iter(), IndexAddOption::DEFAULT, None)?;
    }
    index.write()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let head = repo.head()?;
    let ref_name = head.name();
    let parent_commit_res = head.peel_to_commit();
    let parent_commit = if parent_commit_res.is_ok() {
        vec![parent_commit_res.as_ref().unwrap()]
    } else {
        vec![]
    };

    repo.commit(
        ref_name,
        &signature,
        &signature,
        msg,
        &tree,
        &parent_commit,
    )?;
    Ok(())
}
