# ool
Doubly-supported by NeXTJS and Rust. 

## Front
Find in `app/` is a NeXTJS app governed by [Biome](https://biomejs.dev/) for formatting and linting. Run this pipeline every time you make a change to the app. Should be built in TypeScript, is a multi-page app using [Pages Router](https://nextjs.org/docs/pages). 
Naming conventions are camelCase, and packages should be pushed individually **BEFORE** large changes (feat.,), and with if small changes (hotfixes).

## Back
Find in `back/` is a Rust crate that is governed by [fmt](https://doc.rust-lang.org/std/fmt/). Run this formatting every time that you are about to commit, using `cargo fmt`. Naming conventions is snake_case, and crate chores should be always be pushed first-order.
