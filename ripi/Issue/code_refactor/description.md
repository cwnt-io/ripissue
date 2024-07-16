# code_refactor (Issue)

<!-- toc -->

- [doing](#doing)
- [todo](#todo)
- [specs](#specs)
- [done](#done)

<!-- tocstop -->

## doing

- remove [ignore] from tests
- continue at: test_cli_init
- ripissue.toml config file

implement this builder pattern

```rs
let settings = config::Config::builder()
    .add_source(config::File::with_name("ripissue"))
    .add_source(config::Environment::with_prefix("RIPISSUE"))
    .build()?;

let cfg: Config = settings.try_deserialize()?;
```


- ripi init command
- create a config file
- create Context struct
  - hold project pwd
- create issue integration test

## todo

- Review commands docs/comments
- slugify:
  - Convert the issue name to lowercase:
  - Replace spaces with hyphens (-):
  - Remove special characters and punctuation:
  - Limit the length of the directory name:
  - Handle duplicate issue names:
  - Max length 100 chars
- auto create RIPISSUE.md

## specs

- make it work along side pre-commit hooks
  - should add, but it never should commit if pre-commit fails
    - always check for pre-commits before each operation
- just issues (remove sprints and other elements)
- Refactor code and architecture.
- TDD.

## done

- Context::new with tests
- add anyhow to integration tests
- [x] slugify function
