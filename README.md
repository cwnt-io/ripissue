# ripissue

<!-- toc GFM -->

+ [draft](#draft)

<!-- toc -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

- https://github.com/driusan/bug
- https://github.com/driusan/PoormanIssueTracker


# draft

https://doc.rust-lang.org/std/fs/fn.rename.html

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::rename("a.txt", "b.txt")?; // Rename a.txt to b.txt
    Ok(())
}
```
