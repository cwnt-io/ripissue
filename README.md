# Refactor -> `v0.2.0`

- `develop` branch: will hold `v0.1.X` (temporarily)
- `master` branch: where the magic happens (mainline for `v0.2.X` refactor).

For more details see [[ripissue#`v0.2.X` ROADMAP]].

---

# ripissue

May your issues rest in peace 🪦 ✝️

<!-- toc -->

- [Install](#install)
- [Basic usage](#basic-usage)
  - [Help](#help)
  - [Create an issue](#create-an-issue)
  - [Commit/update an issue](#commitupdate-an-issue)
  - [Close an issue](#close-an-issue)
  - [Reopen an issue](#reopen-an-issue)
  - [List all issues](#list-all-issues)
  - [Close an issue and automatically close the issue branch](#close-an-issue-and-automatically-close-the-issue-branch)
- [Ripissue development](#ripissue-development)
  - [Development Workflow](#development-workflow)
  - [Release Workflow](#release-workflow)
  - [Prepare releases](#prepare-releases)
  - [Create and push tags](#create-and-push-tags)
  - [Publish at crate.io](#publish-at-crateio)
- [Changelog](#changelog)

<!-- tocstop -->

Ripissue is a command-line interface (CLI) tool specifically designed to streamline issue tracking within Git repositories. It introduces a simple file structure convention and workflow, enabling efficient management of issues, tasks, sprints, epics, and initiatives. Utilizing just the git system and filesystem, Ripissue facilitates distributed issue management, making it a versatile tool for developers leveraging Git's distributed nature.

Inspired by:

-   https://github.com/driusan/bug
-   https://github.com/driusan/PoormanIssueTracker


## Install

- Pre-requisite: Ensure Rust and Cargo are installed on your system.
- To install globally in your machine, just run the command:

```sh
cargo add ripi
```

## Basic usage

- The `ripi` command must be executed inside your project at the root dir (where `.git` dir is located)
- The project must be a git repository (`ripi` can automatically create commits and add file changes to it)

### Help

Running `ripi help` gives you the basic structure of the commands and a description of its usage:

```sh
ripi help
```

```
CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Usage: ripi <COMMAND>

Commands:
  issue       Tasks, bugs, features, stories, Pull Requests (PR's), etc. A unit of work
  sprint      Set of issues to be executed in a certain period of time
  epic        Major feature. Can be a set of sprints and/or issues
  initiative  Major abstract long term goal. E.g.: solve all pending bugs until the end of the year
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The `help` is available for every sub-command, so you can consult the available options on-the-fly:

```sh
ripi issue help
```

```
Tasks, bugs, features, stories, Pull Requests (PR's), etc. A unit of work

Usage: ripi issue <COMMAND>

Commands:
  create  Creates a new Issue
  commit  Commits item to git
  close   Closes, adds and commits an item
  reopen  Reopens an item and adds and commits to git
  delete  Deletes an item
  list    Lists all items
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

It works even for commands inside other commands:

```sh
ripi issue create help
```

```
Creates a new Issue

Usage: ripi issue create [OPTIONS] <NAME> [COMMAND]

Commands:
  assign-to  
  help       Print this message or the help of the given subcommand(s)

Arguments:
  <NAME>
          Gives a name to the issue. An ID will be generated from this name

Options:
  -t, --tags <TAGS>
          Tags with this item

  -s, --status <STATUS>
          Status to this item

          Possible values:
          - todo:            Item must be done and is waiting to begin
          - doing:           Item is in execution
          - review-pending:  Item is waiting to be reviewed
          - review-ongoing:  Item is being reviewed
          - review-approved: Item is being reviewed

  -d, --dry
          Do not add/commit it to git

  -b, --branch
          Creates or Switches to a branch related with this item

  -a, --add
          Add all changes to git (git add -A)

  -h, --help
          Print help (see a summary with '-h')
```

### Create an issue

```bash
ripi issue create "My issue name" \
  --tags web3 --tags backend \
  --status todo --branch --add
```

This command will create the following directories and files:

```sh
└── ripi
  └── Issue
    └── my_issue_name
        ├── description.md
        ├── status
        │  └── todo
        └── tags
          ├── backend
          └── web3
```

Also, Ripissue will add all changed files (`--add` flag) and commit those changes and name the commit automatically:

```sh
git log
```

```
commit ba5267fb7412a41d36b4fb080b7a2509013d238a

    (created) Issue #my_issue_name.
```

As we applied the `--branch` flag, ripissue will automatically create and checkout a new branch where you are supposed to implement the issue changes.

```sh
git branch
```

```
* I-my_issue_name
  master
```

So, Ripissue is just a cli helper to apply filesystem structures and conventions to implement a project management and issue tracker workflow integrated to git.

### Commit/update an issue

```bash
ripi issue commit my_issue_name -s doing --add
```

This command will automatically:

- change the status from `todo` to `doing` (just a filename change)
- `--add`: add all changes to git staging area
- create and name the commit message with the issue name

```sh
git log
```

```
commit fce999c51723ad592da1221bb2fa176b442fd3ab

    (up) Issue #my_issue_name.

commit ba5267fb7412a41d36b4fb080b7a2509013d238a

    (created) Issue #my_issue_name.
```

Ripissue just deals with files and directories. Every operation done by `ripi` command can be manually done or adjusted, just by changing file names, locations or content and commiting it to git.

### Close an issue

```bash
ripi issue close my_issue_name
```

A issue closed is just a issue dir that is now located at the `ripi/.closed` location:

```sh
└── ripi
  ├── .closed
  │  └── Issue
  │     └── my_issue_name
  │        ├── description.md
  │        ├── status
  │        │  └── doing
  │        └── tags
  │           ├── backend
  │           └── web3
  └── Issue
```

And it also register this in a git commit too:

```sh
git log
```

```
commit 95b39629f02b25f4f505e2cc83a6498d992a00cd

    (closed) Issue #my_issue_name.

commit 673ba3aadb8b0c2daefea685b55d1fa0104763d8

    (up) Issue #my_issue_name.

commit fce999c51723ad592da1221bb2fa176b442fd3ab

    (up) Issue #my_issue_name.

commit ba5267fb7412a41d36b4fb080b7a2509013d238a

    (created) Issue #my_issue_name.
```

### Reopen an issue

To reopen an issue, just run the command:

```bash
ripi issue reopen my_issue_name
```

And this will move the issue out of the `.closed` dir:

```
└── ripi
  ├── .closed
  │  └── Issue
  └── Issue
    └── my_issue_name
        ├── description.md
        ├── status
        │  └── doing
        └── tags
          ├── backend
          └── web3
```

Also, a new commit was created:

```
commit 28ba5b7bf4b5ade1e170c9fe4bfd111637fd10b1

    (reopened) Issue #my_issue_name.
```

### List all issues

```bash
ripi issue list -a
```

### Close an issue and automatically close the issue branch

There are some steps necessarily to work correctly with an workflow where you dedicate a git branch to work exclusivelly on a particular issue.

Let's suppose we are working on a branch created by ripissue to work on `my_issue_name`:

```
❯ git branch
* I-my_issue_name
  master
```

As we are changing the code within this branc, we will keep commiting the changes with the `ripi` command:

```sh
ripi issue commit my_issue_name -a
```

When we decide that the issue is completed, we can commit our last change with `ripi issue commit`, and then change to out principal branch so we can merge the code:

```sh
git switch master
git merge I-my_issue_name
```

When our changes are correctly merged, we are ready to close our issue and let ripissue deletes our issue branch too:

```sh
# at the `master` branch
ripi issue close my_issue_name -ab
```

Where:

- `-a/--add`: will make shure that all the changes are added to git before the final issue commit
- `-b/--branch`: will delete the specific issue branch (`I-my_issue_name`) after the issue is closed

So, after this command, this should be the `ripi` file structure:

```
└── ripi
  ├── .closed
  │  └── Issue
  │     └── my_issue_name
  │        ├── description.md
  │        ├── status
  │        │  └── doing
  │        └── tags
  │           ├── backend
  │           └── web3
  └── Issue
```

Where the issue `my_issue_name` is now closed. And its respecte branch is now deleted:

```sh
❯ git branch
* master
```

Also, every change we made, are commited automatically to git:

```
commit 856d8e5342b6392ab8519c61363ba7674d583247

    (closed) Issue #my_issue_name.

commit a9d3f59924522fa19e4d2c2b69478dfa518bc802

    (up) Issue #my_issue_name.

commit 28ba5b7bf4b5ade1e170c9fe4bfd111637fd10b1

    (reopened) Issue #my_issue_name.

```

## Ripissue development

The development of `ripissue` utilizes `ripissue` itself 😁.

### Development Workflow

- [cwnt.io's Git Development Workflow](https://github.com/cwnt-io/mgmt/blob/master/git-workflow.md)

### Release Workflow

Inspired by [this release workflow](https://github.com/nextest-rs/nextest/blob/main/internal-docs/releasing.md)[^1].

Releases depend on:

- [cargo-release](https://github.com/crate-ci/cargo-release)
- sign tag?
- be a ripissue mantainer at cwnt-io

### Prepare releases

At `develop` branch:

- pull `fork-executor/develop`
- Prepare [[changelog]]
- `cargo release rc --no-publish -x` (will not publish)
  - to bump minor version and add `rc` to it

### Create and push tags

At `staging` branch:

- pull/fetch/rebase `origin/staging`
- merge with **TAG** created at `develop`
- Tests / minor adjustments / Solve conflicts / Run git hooks
- `cargo release patch --no-publish -x`

At `master` branch:

- pull/fetch/rebase `origin/master`
- merge with **TAG** created at `staging`
- `git push origin --mirror` (branches, tags, all)

### Publish at crate.io

`cargo publish` or `cargo release publish`

## Changelog

See [CHANGELOG](CHANGELOG.md) [^1] [^2]

<!-- references -->

[^1]: https://github.com/nextest-rs/nextest/blob/main/internal-docs/releasing.md "How to perform nextest releases"
[^2]: https://keepachangelog.com/en/1.1.0/ "keep a changelog"
