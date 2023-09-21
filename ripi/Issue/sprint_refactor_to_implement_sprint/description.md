# sprint_refactor_to_implement_sprint (Issue)

<!-- toc GFM -->

* [Comments](#comments)

<!-- toc -->

- [ ] impl functions to add/remove issues from sprint
    ```sh
    ripi sprint edit [sprint_id] add-issue --repo repo_name --issue issue_name
    ripi sprint edit [sprint_id] remove-issue --repo repo_name --issue issue_name
    ```
- [x] impl specific sprint create
    ```sh
    ripi sprint create [due-date] # validate iso-date for name
    ```
- [x] impl specific issue create
- [x] setup my (gubasso's) rustfmt in nvim
- [x] refactor app to be composable
    - [x] flatten to args to make this api work ↴

## Comments

> **gubasso**
> 
> Issue and branch renamed from `implement_project_element` to `sprint_refactor_to_implement_sprint`
>
> Will no longer implement the Project element/functionality. But now the focus is to refactor Clap structures and
> implement the Sprint functionality that associates it with issues from another respositories

> **isma**
> 
> Ok. You are right! It's approved.

> **gubasso**
> 
> I've opened an issue to improve list output format
> But I think that at this point of development this functionality is ok for now


> **isma**
> 
> Gustavo, your code is awesome!
>
> I think we should improve the list output format.
