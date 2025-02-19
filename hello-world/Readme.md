



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



cargo install bunyan

TEST_LOG=true cargo test health_check_works | bunyan