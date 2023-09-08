# ripissue

<!-- toc GFM -->

+ [todo:](#todo)
+ [draft](#draft)

<!-- toc -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

- https://github.com/driusan/bug
- https://github.com/driusan/PoormanIssueTracker

# todo:

git add _1_todo/new_issue_2/
git add _0_backlog/new_issue_2
- implement renaming (moving) issue (see links at readme.md)
issue.to_kanban(kanban_enum)
- mv in fs and returns new issue

- publish on crates.io
    - https://doc.rust-lang.org/cargo/reference/publishing.html

# draft

https://doc.rust-lang.org/std/fs/fn.rename.html

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::rename("a.txt", "b.txt")?; // Rename a.txt to b.txt
    Ok(())
}
```
