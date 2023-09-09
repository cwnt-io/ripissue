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

- wip_issue ->
    - add and commit that issue (msg will be the kanban status)
- wip -> doing

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
