# Bandsaw - Rubric CSV Creator
## Description
Bandsaw is a simple webapp that creates a single CSV from a Google Sheet and Canvas CSV.
This CSV is used in the evals website to create rubrics for a class of students.

## Prerequisites
A [Rust][rust] installation is required. Install `rustup-init` and run it:
```sh
$ brew install rustup-init
$ rustup-init
```

[Trunk][trunk] is used for serving/building. Install it using `cargo`:
```sh
$ cargo install --locked trunk
```

[rust]: https://rust-lang.org
[trunk]: https://trunkrs.dev

## Running
Serve the webapp on localhost:
```sh
$ trunk serve
```

Build the webapp for production:
```
$ trunk build --release
```

## Tasks
* Implement tests for CSV output
* Add compatibility for other Google Sheet formats
* Show popup when an internal error occurs
