# ripissue

<!-- toc -->

- [Development Workflow](#development-workflow)
- [Install](#install)
- [Basic usage](#basic-usage)
  - [Create an issue](#create-an-issue)
  - [Commit/update an issue](#commitupdate-an-issue)
  - [Close an issue](#close-an-issue)
  - [Reopen an issue](#reopen-an-issue)
  - [List all issues](#list-all-issues)
- [Changelog](#changelog)
- [References](#references)

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

## Changelog

See [CHANGELOG]('./CHANGELOG.md')[^1][^2].

## References

[^1]: https://github.com/nextest-rs/nextest/blob/main/internal-docs/releasing.md "How to perform nextest releases"
[^2]: https://keepachangelog.com/en/1.1.0/ "keep a changelog"


