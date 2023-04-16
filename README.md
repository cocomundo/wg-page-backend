# WG Page Backend

## Backend

To run the backend for development, start a postgres Database

```sh
docker pull postgres
docker run --name postgresDB \
    -p 5432:5432 \
    -e POSTGRES_USER=user \
    -e POSTGRES_PASSWORD=1234 \
    -e POSTGRES_DB=wg_db \
    -d postgres
```

create a .env file

```sh
DATABASE_URL=postgres://user:1234@localhost/wg_db
```

install diesel_cli and initialize the database:
```sh
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

then start the server (The backend is the default binary)

```sh
cargo run [-- --address localhost:8000]
```

optionally, set a logging level:

```sh
RUST_LOG=info cargo run [-- --address localhost:8000]
```

## Deployment (TBD)

## Planned
#### Website features
- user authentication

#### Categorys
- welcoming
- shopping list
- cleaning list
- calender
- gallery
- guestbook


#### DB content:
- Users
- shopping items
- appointments
