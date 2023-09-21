# sprint_refactor_to_implement_sprint (Issue)

<!-- toc GFM -->

* [Comments](#comments)

<!-- toc -->

- [ ] refactor app to be composable
    - [ ] flatten to args to make this api work ↴
        ```sh
        ripi sprint create [due-date] # validate iso-date for name
        ripi sprint add-issue --repo repo_name --issue issue_name
        ripi sprint remove-issue --repo repo_name --issue issue_name
        ```

- [x] setup my (gubasso's) rustfmt in nvim


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
