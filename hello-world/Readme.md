



```
./scripts.sh/init_db.sh
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```


Bezi dzelislerden sonra
```
 SKIP_DOCKER=true ./scripts.sh/init_db.sh 
```