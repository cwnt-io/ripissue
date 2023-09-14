# cmd_list_add_functionalities_and_filters (ISSUES)

- [x] get_all_elems working
- [ ] prints by filter
    - [ ] filter by status
    - [ ] filter by tags, tag groups

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
- [x] V 
```
struct Issues
let issues: <Id, Issue> = Issue::get_all(&[ElemPath]);
```
