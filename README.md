# ws-exploration


# Preview



https://github.com/zvup/ws-exploration/assets/121556153/3e8d3589-d0f4-4f00-9322-185604f78998


## Concept
* Send messages/ control client via a user interface on the web

## Reproduce

```shell
sh b.sh

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

```sql
CREATE table usertable(
   username text,
   password text,
   key text,
   auth text
);


```




