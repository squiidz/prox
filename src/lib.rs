#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result;
use dotenv::dotenv;
use std::env;

use models::{User, NewUser};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &PgConnection, name: String, age: i32) -> User {
    use schema::users;

    let new_user = NewUser{name: name, age: age};

    diesel::insert(&new_user).into(users::table)
        .get_result(conn)
        .expect("Error saving user")
}

pub fn get_user(conn: &PgConnection, idx: i32) -> User {
    use schema::users::dsl::*;

    let mut result = users.filter(id.eq(idx))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading user");
    result.swap_remove(0)
}

pub fn get_all_user(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    let result = users.load::<User>(conn)
        .expect("Error loading users");

    result
}
pub fn update_user(conn: &PgConnection, user: User) -> User {
    use schema::users::dsl::*;

    let result = diesel::update(users.filter(id.eq(id)))
        .set(name.eq(user.name))
        .get_result::<User>(conn)
        .expect("User not found");

    result
}

pub fn delete_user(conn: &PgConnection, idx: i32) -> Result<usize, result::Error> {
    use schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(idx)))
        .execute(conn)
}

pub fn delete_all_user(conn: &PgConnection) -> Result<usize, result::Error> {
    use schema::users::dsl::*;

    diesel::delete(users).execute(conn)
}
