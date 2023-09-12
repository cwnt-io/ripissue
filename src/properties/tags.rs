use crate::helpers::slug_tag;

#[derive(Debug, Clone)]
pub struct Tag(Vec<String>);

impl Tag {

    pub fn new(s: &str) -> Self {
        let s = slug_tag(s);
        Self(s.split("-").map(|p|p.to_owned()).collect())
    }

    pub fn to_str(&self) -> String {
        self.0.join("-")
    }


}

#[derive(Debug, Clone)]
pub struct Tags(Vec<Tag>);

impl Tags {

    pub fn new(tags_vec: Vec<Tag>) -> Self {
        Tags(tags_vec)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Tag> {
        self.0.iter()
    }

    pub fn from_vec_str(v: &Vec<String>) -> Self {
        let tags_vec: Vec<Tag> = v.iter()
            .map(|e| Tag::new(e)).collect();
        Self(tags_vec)
    }

}
