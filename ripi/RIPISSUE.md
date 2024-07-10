# RIPISSUE

<!-- toc -->

- [`v0.2.X` ROADMAP](#v02x-roadmap)
  - [Cwntflow refactor](#cwntflow-refactor)
  - [Code refactor](#code-refactor)
  - [AI integration](#ai-integration)
    - [research](#research)
    - [auth flow](#auth-flow)
    - [AI auto-complete](#ai-auto-complete)
  - [`list` command](#list-command)
  - [`chat` command](#chat-command)
  - [`changelog` generator](#changelog-generator)
  - [Auto update RIPISSUE.md](#auto-update-ripissuemd)
  - [Launch: Final adjustments](#launch-final-adjustments)
- [Todo Backlog Draft](#todo-backlog-draft)

<!-- tocstop -->

## `v0.2.X` ROADMAP

This roadmap aims to deliver a production ready version for the [Backdrop Build v5](https://backdropbuild.com/v5/) program.

Definitions

- Commit messages must have this structure: https://www.conventionalcommits.org/en/v1.0.0/
  ```
  <type>[optional scope]: <description>

  [optional body]

  [optional footer(s)]
  ```
- A "one-liner" commit message must have this format:
  ```
  <type>[optional scope]: <description>
  ```

Deliverables:

- ✅ [Cwntflow refactor](#cwntflow-refactor)
- ⏳ [Code refactor](#code-refactor)
- ⏳ [AI integration](#ai-integration)
  - ⏳ [research](#research)
  - ⏳ [auth flow](#auth-flow)
  - ⏳ [AI auto-complete](#ai-auto-complete)
- ⏳ [`list` command](#list-command)
- ⏳ [`chat` command](#chat-command)
- ⏳ [`changelog` generator](#changelog-generator)
- ⏳ [Auto update RIPISSUE.md](#auto-update-ripissuemd)
- ⏳ [Launch: Final adjustments](#launch-final-adjustments)

### Cwntflow refactor

- Refactor Cwnt's workflow based on a trunk based development model.
- Document the workflow.
- Make it powered by Ripissue.

### Code refactor

- Refactor code and architecture.
- TDD.
- auto create ripissue.md
- make it work along side pre-commit hooks
  - should add, but it never should commit if pre-commit fails
    - always check for pre-commits before each operation

### AI integration

1) code
2) ai generate commit messages from your **staged** changes 
  - gitdiff + prompt > commit message from AI ----> input of $EDITOR
                                                \-> append to ai-changelog file

- [research](#research)
- [auth flow](#auth-flow)
- [ai helper with a commit without an issue](#ai-helper-with-a-commit-without-an-issue)
- [ai helpers with an issue in place](#ai-helpers-with-an-issue-in-place)

#### research

- https://github.com/m1guelpf/auto-commit
- research solutions/apis (use openai? is there another? if we can find a backdropbuild partner, it would be better)
  - See a partner that might be a good fit for your project? Hit "Connect"
  - https://backdropbuild.com/v5/partners
    - https://backdropbuild.com/v5/partners/modal
    - https://backdropbuild.com/v5/partners/langchain
- prompt engineering

#### auth flow

- like ansible-vault

```sh
ripi issue ... --openai-token "<token>"
ripi issue ... --openai-token-file my_token_file
```

- env variable: RIPISSUE_API_AI_TOKEN

- config_file.toml

```toml
[openai]
token_file = "path/to/token_file/or/script"
```
- token file

`my_token_file` (option 1)

```
<token>
```

`my_token_file` (option 2)

```sh
#!/bin/bash
gopass my_path/token
```

#### AI auto-complete

##### Commit NO issue

Helper with a commit WITHOUT an issue

```sh
ripi commit --openai-token "<token>"
```

- no commit message or issue id specified
- options (those options can combine with each other):
  - one line commit message
  - detailed topics of commit changes
    - detailed topics at the commit message itself

##### Commit WITH issue assigned

Helpers with an issue in place.

Commit message with an issue in place. Issue is set.

The commit message itself will always be "one-liner".

Both options can work together (in combination):

- [[#One-liner: complement commit message]]
- [[#Full-log: append full log message to a file]]

###### One-liner: complement commit message

- ai will complement the commit messagge with a brief description

```sh
ripi commit <issue_id> --ai-complement-commit-message
```

Commit message format:

```
<type>[optional scope]: <issue_id> (<ai-description-very-short>)
```

- `ai-description-very-short`: few words, just to have a visual cue from the git log to see what that commit is about.

###### Full-log: append full log message to a file

```sh
ripi commit <issue_id> --ai-changelog
```

`ripi/Issues/<issue_id>/ai-changelog.md`

```md
# <log-header>

[...]

# <log-header>

- Updated the cover letter reference links in README.md to include more detailed URLs.
- Added new submission templates for Prisma Data General Applications Engineering.
- Added a new cover letter and job description for Prisma Data Senior Software Engineer.
- Updated the Prisma Data Software Engineer README and cover letter PDF.
```

- `log-header`
  - template
    ```
    <ai-description-very-short> - <author-email> - <now-iso-date>
    ```
  - example
    ```md
    ## <ai-description-very-short> - root@cwnt.io - 2024-07-09T08:54:15-03:00
    ```

### `list` command

- ripi issue list, make it better
  - pure (to be machine used)
  - ascii art, visual
  - kanban view (by tags/status)

### `chat` command

- chat:
  - `-m "my chat message"`
  - open in $EDITOR

### `changelog` generator

- ai changelog?
- https://keepachangelog.com/en/1.1.0/
- changelog generator: https://git-cliff.org/
  - crud (manual): add + update + remove

### Auto update RIPISSUE.md

- auto update ripissue.md
  - `<!--ripissue:open-->`: list all opened issues
    - `:close`
    - `:all`
  - when: list/open/close issues

### Launch: Final adjustments

- refactor readme (new logo)
- video launch
  - short demo video (< 3 mins)
  - show what you build
  - a way for people to actually try out what you built
  - what problem you try to solve

---

## Todo Backlog Draft

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

