# sprint_refactor_to_implement_sprint (Issue)

- [ ] impl functions to add/remove issues from sprint
    ```sh
    ripi sprint manage [sprint_id] add-issue --repo repo_name --issue issue_name
    ripi sprint manage [sprint_id] remove-issue --repo repo_name --issue issue_name
    ```
- [x] impl specific sprint create
    ```sh
    ripi sprint create [due-date] # validate iso-date for name
    ```
- [x] impl specific issue create
- [x] setup my (gubasso's) rustfmt in nvim
- [x] refactor app to be composable
    - [x] flatten to args to make this api work ↴
