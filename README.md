# helo this is some kind of app

[![build + test](https://github.com/Patrick2402/rust_webpage/actions/workflows/rust.yml/badge.svg)](https://github.com/Patrick2402/rust_webpage/workflows/rust.yml)
## TODO:
```
# huge todo: make the website usable
# for reference lokk at implementation of login how to handle useless functionality
```


### The original sin: combining diesel and sqlx for extra dependency count
```
cargo install diesel_cli --no-default-features --features postgres
brew install libpq
brew link --force libpq
```

### important when updating the schema
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres

cargo sqlx prepare -- --all-targets --all-features
```

quick fix 1:

```
PQ_LIB_DIR="$(brew --prefix libpq)/lib"
```