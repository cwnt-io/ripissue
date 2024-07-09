# changelog_generator (Issue)

Create my changelog generator (inside ripissue? or separated project that I will integrate with ripissue?)?

Get inspired to see the issues with the flow of those tools and/or see the features

- videos:
  - [The long introduction of non-destructive changelog generation (cargo smart-release 0.4) - Byron's Devlog](https://www.youtube.com/watch?v=a4CzzxJ7ecE)
  - [Release-plz: releasing crates like it's 2023 - Marco Ieni - RustLab Conference](https://www.youtube.com/watch?v=kXPBVGDkQSs)
- Search for `changelog` at crates.io
  - see each project

- two only sources of truth (will be both):
  - kv map
  - commit history
- feature: kv map where:
  - k is the commit in the history
    - k may have many fields to mach (by priority)
      - commit hash
      - text message (regex pattern)
  - v is the "edited" (new) commit message
    - will have the correct structure (pattern) of conventionalcommits (toml with the correct structure)
- outputs
  - templates
    - allow user to create a template
  - models:
    - changelog https://keepachangelog.com/en/1.1.0/
    - debian changelog
    - others?
