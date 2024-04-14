# How to run 

```sh
cargo run -- --help 
cargo run -- list
cargo run -- add username001 password
cargo run -- add --admin true username001 password // for grant permission admin
cargo delete -- admin
cargo delete -- username001
cargo run -- change-password admin P@ssword

# For --help
cargo run -- --help
cargo run -- add --help
cargo run -- change-password --help
cargo run -- delete --help
```