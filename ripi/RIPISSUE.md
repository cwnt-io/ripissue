# RIPISSUE

<!-- toc -->

- [Todo Backlog Draft](#todo-backlog-draft)

<!-- tocstop -->

# Todo Backlog Draft

- general refactor + tests
- auto create ripissue.md
- auto update ripissue.md
  - `<!--ripissue:open-->`: list all opened issues
    - `:close`
    - `:all`
  - when: open/close issues

- [ ] integrate ripissue with:
  - https://github.com/MarcoIeni/release-plz
    - !! https://www.conventionalcommits.org/en/v1.0.0/
    - https://git-cliff.org/
  - [poc-changelog-generator] ?
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

