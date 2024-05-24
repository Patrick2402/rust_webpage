# helo this is some kind of app

[![Quality Gate Status](http://18.193.102.105:9000/api/project_badges/measure?project=Test&metric=alert_status&token=sqb_804b64e0027f0d32df4ac3e0f845aee0a6d35a75)](http://18.193.102.105:9000/dashboard?id=Test)
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
