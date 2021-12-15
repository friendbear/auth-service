##

create database auth_service

```
docker run --name some-postgres --network database -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```

```
docker run -it --rm --network database postgres psql -h some-postgres -U postgres
create database auth_service

```

or 

```
createdb -h localhost -p 5432 -U postgres auth_service
```