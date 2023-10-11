# automatic_create_branch (Issue)

- branch create/switch/delete with -b flag


- [ ] improve close issue flow `fn close`
  - `Elem::commit`:
    - the important part of this code is the initial one when setting up the elem, before commit
    - extract this piece to another method
    - do not call Elem::commit inside `fn close`
    - do not replicate the `branch` workflow from `fn commit` inside `fn close`

- [x] test commands
  - [x] create
    - [x] again
  - [x] commit
  - [x] close
  - [x] delete
  - [x] reopen
- [x] // TODO

