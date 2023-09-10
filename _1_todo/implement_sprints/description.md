# implement_sprints

<!-- toc GFM -->

+ [issues:](#issues)
+ [opt 1: one file `issues` inside each sprint dir](#opt-1-one-file-issues-inside-each-sprint-dir)
    * [sprint subcommand:](#sprint-subcommand)

<!-- toc -->

# issues:

- [ ] subcmd_sprint_create

# opt 1: one file `issues` inside each sprint dir

```
/mgmt
    /sprints
        /sprint_1
            issues
        /sprint_2
```

at `issues` file, a list of issues for that sprint

```
<repo>-<issue_name>
<repo>-<issue_name>
<repo>-<issue_name>
<repo>-<issue_name>
```

## sprint subcommand:

- sprint list ...
    - list all sprints
    - --issues: list all sprints and its issues
- sprint list <sprint_name>
    - list all issues for that sprint

- sprint status <sprint_name>
    - returns all issues and its statuses/kanban

issue_name/sprints
