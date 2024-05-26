## Setup a db
Run docker compose
```bash
docker compose up
```
or for detached mode
```bash
docker compose up -d
```
If you want to force a full restart including deleting the data:
```bash
docker compose down --volumes    
docker compose up
```
Find the connection ip for the database (only necessary if you want to play with it in pgAdmin )
```bash
docker inspect {container-id}
```
Then connect to the ip via pgAdmin if you would like to inspect the database.

### Notes
Currently this also includes pgAdmin for debugging the db.   
If you dont need it just remove the pgAdmin service from the docker-compose file.


