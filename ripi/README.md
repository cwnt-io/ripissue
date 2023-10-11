# ripi

<!-- toc -->

- [Todo Backlog Draft](#todo-backlog-draft)

<!-- tocstop -->

# Todo Backlog Draft

- improve close issue flow `fn close`
  - `Elem::commit`:
    - the important part of this code is the initial one when setting up the elem, before commit
    - extract this piece to another method
    - do not call Elem::commit inside `fn close`
    - do not replicate the `branch` workflow from `fn commit` inside `fn close`

-   [ ] implement tests (unit + integration)
-   [ ] `ripi create` automatically creates a branch too?
    -   check if curr branch is `develop`
    -   branch-name == issue/sprint/epic/initiative id
