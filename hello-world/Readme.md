



```
./scripts.sh/init_db.sh
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```

psql yuklemek ucun
```
sudo apt install postgresql-client
```

Bezi dzelislerden sonra
```
 SKIP_DOCKER=true ./scripts.sh/init_db.sh 
```q

RUST_LOG=debug cargo run

```
cargo install bunyan

TEST_LOG=true cargo test health_check_works | bunyan
```
cargo vendor ile asililiqlari cixartdim
```
To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```


```
docker build --tag zero2prod --file Dockerfile .
```


docker run -p8000:8000 --network=host  zero2prod


ulimit -n 10000

cargo test -- --nocapture



# sqlx logs are a bit spammy, cutting them out to reduce noise
```
export RUST_LOG="sqlx=error,info"
export TEST_LOG=true
cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan

```

sqlx migrate add create_users_table

sqlx migrate add rename_password_column

sqlx migrate add seed_user



10.7.5.4
Reject Unauthenticated Users 494