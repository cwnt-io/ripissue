# cmd_commit_adjust_tags_and_status_behaviour (ISSUES)

- [x] tags
// TODO: is not set, but append
// issue.append_tags(Tags::from_vec_str(&ts))

- [x] status:
// TODO: remove old and add new
issue.set_status(Some(s.clone()));
    - rename if is None
