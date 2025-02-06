# ool
Doubly-supported by NeXTJS and Rust. Should just be testing GitHub REST API using various methods.

## Front
Find in `app/` is a NeXTJS app governed by [Biome](https://biomejs.dev/) for formatting and linting. Run this pipeline every time you make a change to the app. Should be built in TypeScript, is a multi-page app using [Pages Router](https://nextjs.org/docs/pages). 
Naming conventions are camelCase, and packages should be pushed individually **BEFORE** large changes (feat.,), and with if small changes (hotfixes).

## Back
Find in `back/` is a Rust crate that is governed by [fmt](https://doc.rust-lang.org/std/fmt/). Run this formatting every time that you are about to commit, using `cargo fmt`. Naming conventions is snake_case, and crate chores should be always be pushed first-order.

## Testing
You'll find that tests are abundant within both areas. The front contains basic (unit tests), if is calls to check if web server returns as-should calls. The back contains both basic & integration tests to keep us on our toes - as we should be constantly updating data from GitHub's repository to keep getting the latest data.  

Unit test conventions should be same naming conventions, but with `test_*` and `*_{type}` at the end of the test. 
`test_get_recent_commits_success` where `test_` is the prefix, `*_{type}` would be `success`, as that's what we're looking for. As-is, a simple basic test.  

Integration tests are a bit more complex, where we continue with the `test_` prefix, but instead of `*_type`, we'll be looking for `*_{expected_behavior}`. 
`test_github_full_repo_refresher_is_not_empty` where `test_` is the prefix, and `_is_not_empty` is the expected behavior that the test should be looking for. 
