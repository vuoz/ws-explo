# ws-exploration


# Preview


https://github.com/user-attachments/assets/dcee4c2a-64c8-4a05-8620-2f9ddebe53aa




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
### Db Connection
Please create a .docker.env file and paste the following line. This is the connection string for the pg database.
```sql 
DATABASE_URL=postgresql://myuser:yourpass@localhost:6000/mydb
```


