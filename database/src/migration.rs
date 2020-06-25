use postgres::{Client, NoTls};
use refinery::embed_migrations;
use std::env;

embed_migrations!("./migrations");

pub async fn run() -> () {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut client = Client::connect(&database_url, NoTls).expect("could not connect to database");

    migrations::runner()
        .run(&mut client)
        .expect("could not run migrations");
    ()
}
