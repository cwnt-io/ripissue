# ripissue

<!-- toc GFM -->

+ [todo:](#todo)

<!-- toc -->

CLI tool written in Rust for distributed bug / issue / story tracking with the filesystem and git.

Inspired by:

- https://github.com/driusan/bug
- https://github.com/driusan/PoormanIssueTracker

# todo:

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
