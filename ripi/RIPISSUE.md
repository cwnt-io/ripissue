# RIPISSUE

<!-- toc -->

- [ai integration](#ai-integration)
- [backdropbuild](#backdropbuild)
- [Todo Backlog Draft](#todo-backlog-draft)

<!-- tocstop -->

## ai integration

- research solutions/apis (use openai? is there another? if we can find a backdropbuild partner, it would be better)
  - See a partner that might be a good fit for your project? Hit "Connect"
  - https://backdropbuild.com/v5/partners
    - https://backdropbuild.com/v5/partners/modal
    - https://backdropbuild.com/v5/partners/langchain

## backdropbuild

- ai generated issue/commit message
  - one line commit message
  - reverse flow: 1) crate code, 2) ai generate commit message from diff
    - gitdiff > + prompt > commit message from AI > input of $EDITOR
  - normal flow: 1) issue already, 2) ai will complement the issue description with the diff for that commit
  - auth like ansible

ripi issue ... --openai-token "<token hard coded>"

ripi issue ... --openai-token-file my_token_file

`my_token_file` (option 1)

```
<token>
```

`my_token_file` (option 2)

```sh
#!/bin/bash
gopass my_path/token
```

(priority, if there is time, come back to implement flags too)
`ripissue_config.toml`

```toml
[openai-token]
file = "my_token_file"
```

ripi issue ... --ai-helper

```
<type>[optional scope]: <issue_id> (<description very short>)
```

`description.md`

```md
# id_da_issue

- tenho que fazer nao sei oqeu
- dasfsdf
- sadfsda
```

`diff-doc.md`

```md
# commit <commit_message>

[...]

# commit <commit_message>

- Updated the cover letter reference links in README.md to include more detailed URLs.
- Added new submission templates for Prisma Data General Applications Engineering.
- Added a new cover letter and job description for Prisma Data Senior Software Engineer.
- Updated the Prisma Data Software Engineer README and cover letter PDF.
```

- ripi issue list, make it better
  - pure (to be machine used)
  - ascii art, visual

- chat:
  - `-m "my chat message"`
  - open in $EDITOR

- https://www.conventionalcommits.org/en/v1.0.0/

- changelog generator: https://git-cliff.org/
  - crud (manual): add + update + remove

branch naming convention

```md
-   **feat: allow provided config object to extend other configs**
  -   Branch Name: `feat/config-extension`
-   **feat!: send an email to the customer when a product is shipped**
  -   Branch Name: `feat/breaking-email-on-shipping`
-   **feat(api)!: send an email to the customer when a product is shipped**
  -   Branch Name: `feat/api-breaking-email-on-shipping`
-   **chore!: drop support for Node 6**
  -   Branch Name: `chore/drop-node6`
-   **feat(lang): add Polish language**
  -   Branch Name: `feat/lang-polish`
```

- general refactor + tests
- after all:
  - refactor readme (new logo)

## Todo Backlog Draft


- auto create ripissue.md
- all generated .md: add a "\n"
- make it work along site pre-commit hooks
  - should add, but it never should commit if pre-commit fails
    - always check for pre-commits before each operation
- auto update ripissue.md
  - `<!--ripissue:open-->`: list all opened issues
    - `:close`
    - `:all`
  - when: open/close issues

- [ ] integrate ripissue with:
  - https://github.com/MarcoIeni/release-plz
  - [[changeloggenerator-issue]]
  - general commit: with a general message (minor updates not necessarily related with a issue)

- [ ] ripissue packaging
  - [ ] [[packagingdebianubuntu-issue]]
  - [ ] [[packagingsnap-issue]]
  - [ ] [[packagingflatpak-issue]]
- [ ] tests: ci/cd -> packages

- [ ] build ripissue homepage

- handle special chars like "/" when slugifying names
  - if issue create has "something / something else" it will create a issue name " something else"
- crates.io and github/ripissue: add owners
- [ ] make a professional README
  - link to crates.io
  - basic usage (simple, just as `bug`)
  - increment usage with minimal single workflow (with branches)
  - more: mgmt repo and sprints
  - directories tree
  - full team workflow
  - release workflow
    - tag workflow: how it works with annotated tags...
- [ ] automatic identify issue from branch
  - if in branch `I-my_issue_id`, command `ripi issue commit` auto identifies id
- [ ] implement tests (unit + integration)
- [ ] `ripi <elem> list`: increment print layout for cmd list: show issues properties to stdout (status, tags, etc...)
- automate CHANGELOG?
  - [A Beginner’s Guide to Git — What is a Changelog and How to Generate it](https://www.freecodecamp.org/news/a-beginners-guide-to-git-what-is-a-changelog-and-how-to-generate-it/)

