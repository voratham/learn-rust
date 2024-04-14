# New project

```sh
cargo new learn-rust # create common project
cargo new --lib authentication # create library
```

# How to run test
```sh
cargo test
```


# How to search package via CLI
```
cargo search sha2
```

# How to import library project rust local directory

```rust
[package]
name = "login"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
authentication = { path = "../authentication" }  // path

```

# Install library

```sh
cd session1/authentication
cargo add serde -F derive  # -F specific feature
cargo add serde_json
```
