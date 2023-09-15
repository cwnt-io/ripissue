# ripissue

<!-- toc GFM -->

+ [todo:](#todo)

<!-- toc -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

- https://github.com/driusan/bug
- https://github.com/driusan/PoormanIssueTracker

# todo:

- [ ] Elements: Sprint, Epic, etc...
- [ ] issue as branch
    - [ ] Option to open a branch when open an issue?
    - [ ] Optino to rebase a branch when close an issue?
- For `Element` and `Issue` I'm using the Self::Item type wrongly
    - it has to be user for specifing the general type of internal element
- Use Git as the backend for chat https://opensource.com/article/19/4/git-based-chat
    - https://github.com/ephigabay/GIC
    - https://git-scm.com/docs/git-notes

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
