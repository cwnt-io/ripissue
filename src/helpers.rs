use slugify::slugify;

pub fn slug(s: &str) -> String {
    slugify!(s, separator = "_")
}
