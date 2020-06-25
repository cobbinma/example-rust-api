# example-rust-api

This is an example Rust REST API using [Tide](https://github.com/http-rs/tide) and [SQLX](https://github.com/launchbadge/sqlx).

To run you must have rust installed.

1. Run `docker-compose up` to start Postgres database
2. Use `cargo run` to start API
3. Make a POST request to `http://localhost:8181/pet` with a request body (shown below) to add a pet
4. Make a GET request to `http://localhost:8181/pets` to get a list of pets
5. Make a GET request to `http://localhost:8181/pet/:id` to get an individual pet 

Example request body to add a pet (an additional tag field is optional)
```json
{
	"id": 1,
	"name": "Ben"
}
```

Make a GET request to `http://localhost:8181/oas` for the full API schema

Use `cargo test` to run tests

![](https://pbs.twimg.com/media/CVwvDn5UkAAlne_?format=png&name=900x900)
