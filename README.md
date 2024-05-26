# ws-exploration


# Preview



## Concept
* Send messages / control client via a user interface on the web

## Reproduce

```shell
cargo leptos watch
```
you will need cargo leptos  to run this project 
```shell
cargo install cargo-leptos
```

## Stack
* Rust
* Axum
* Leptos
* Postgres

## Missing improvements
* improve ui 
* improve flow of the site (auto redirects etc.)
* implement e2ee
* switch to ssr instead of csr


### Db prerequisites

Use this in your existing postgres database 
```sql
CREATE table usertable(
   username text,
   password text,
   key text,
   auth text
);
```
or just create a new database via the docker-compose file in the docker-db folder.
[Docker DB instructions](docker-db/README.md)
### Db Connection
Please create a .env file and paste the following line. This is the connection string for the pg database.
```sql 
DATABASE_URL=postgresql://myuser:yourpass@localhost:6000/mydb
```


