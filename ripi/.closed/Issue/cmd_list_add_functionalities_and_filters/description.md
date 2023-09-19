# cmd_list_add_functionalities_and_filters (ISSUES)

<!-- toc GFM -->

* [todo](#todo)
* [done/old](#doneold)

<!-- toc -->

## todo

- [x] basic print layout
- [x] get all elems, save to struct Elems
- [x] get by filter
    - [x] all (closed / opened)
    - [x] filter by status
    - [x] filter by tags, tag groups

## done/old

```rust
struct Issues
let issues: <Id, Issue> = Issue::get_all(&[ElemPath]);
```
- [x] get_all_elems working
```rust
let issues = get_all_elems::<Issue>()?;
```

- [x] rename base_closed -> base_path_closed
- [x] Issue::base_path_all
- [x] refactor elements
    - [x] get_path / from_path
    - [x] path_or_id
    - [x] fn path() - self.path() - self.path
        - self.path = base + id
    - [x] write_basic_file
    - [x] write
    - [x] ElemPath
    - [x] ElemId
