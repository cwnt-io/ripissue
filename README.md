# ripissue

<!-- toc GFM -->

+ [Development Workflow](#development-workflow)
+ [todo:](#todo)

<!-- toc -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

- https://github.com/driusan/bug
- https://github.com/driusan/PoormanIssueTracker

# Development Workflow

- [cwnt.io Git Workflow](https://github.com/cwnt-io/mgmt/blob/master/git-workflow.md)

# todo:

- [ ] to publish to cargo:
    - [ ] project elem
    - [ ] sprint/epics/initiatives
        - yaml file: https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust

- [ ] refactor app with clap builder
- [ ] increment print layout for cmd list: show issues properties to stdout (status, tags, etc...)
- [ ] clean cwnt github repo (delete popcorn-web-app)
- [ ] clap args inheritance

/repo1
    /_...(issues)
/repo2
/repo3

/mgmt
    /sprints
        /sprint_1
            <repo>-<issue_name>
            <repo>-<issue_name>
        /sprint_2
    /epics
        /epic_1
            sprint_1
            sprint_2
        /epic_2
    /initiatives
        /initiative_1
            /epic_1
            /epic_2
    /projects
        /proj_1
            /repo1
            /repo2
        /proj_2
            /repo3
