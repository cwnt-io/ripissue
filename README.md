# ripissue

May your issues rest in peace 🪦 ✝️

<!-- toc -->

- [Development Workflow](#development-workflow)
- [Install](#install)
- [Basic usage](#basic-usage)
  - [Create an issue](#create-an-issue)
  - [Commit/update an issue](#commitupdate-an-issue)
  - [Close an issue](#close-an-issue)
  - [Reopen an issue](#reopen-an-issue)
  - [List all issues](#list-all-issues)
- [Release Workflow](#release-workflow)
  - [Prepare releases](#prepare-releases)
  - [Create and push tags](#create-and-push-tags)
  - [Publish at crate.io](#publish-at-crateio)
- [Changelog](#changelog)

<!-- tocstop -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

-   https://github.com/driusan/bug
-   https://github.com/driusan/PoormanIssueTracker

## Development Workflow

  -   [cwnt.io Git Workflow](https://github.com/cwnt-io/mgmt/blob/master/git-workflow.md)

## Install

```bash
cargo add ripi
```

## Basic usage

### Create an issue

```bash
ripi issue commit "My issue" -t web3 -t dev-john -s todo
```

### Commit/update an issue

```bash
ripi issue commit "My issue" -s doing
```

### Close an issue

```bash
ripi issue close my_issue
```

### Reopen an issue

```bash
ripi issue reopen my_issue
```

### List all issues

```bash
ripi issue list -a
```

## Release Workflow

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
