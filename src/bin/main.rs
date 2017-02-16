#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

extern crate rocket;
extern crate serde_json;

extern crate prox;

use self::prox::models::*;

use rocket_contrib::{JSON, Value};
use rocket::response::NamedFile;
use rocket::State;

use diesel::pg::PgConnection;

use std::io;
use std::path::{PathBuf, Path};
use std::sync::Mutex;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("dist/index.html")
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).ok()
}

#[get("/api/users/<idx>")]
fn show(connection: State<Mutex<PgConnection>>, idx: i32) -> JSON<User> {
    let conn = connection.lock().unwrap();
    let user = prox::get_user(&conn, idx);
    JSON(user)
}

#[get("/api/users")]
fn show_all(connection: State<Mutex<PgConnection>>) -> JSON<Vec<User>> {
    let conn = connection.lock().unwrap();
    let users = prox::get_all_user(&conn);
    JSON(users)
}

#[post("/api/users/new", format = "application/json", data = "<user>")]
fn create(connection: State<Mutex<PgConnection>>, user: JSON<NewUser>) -> JSON<User> {
    let conn = connection.lock().unwrap();
    let user = prox::create_user(&conn, user.0.name, user.0.age as i32);
    JSON(user)
}

#[put("/api/users", format = "application/json", data = "<user>")]
fn update(connection: State<Mutex<PgConnection>>, user: JSON<User>) -> JSON<Value> {
    let conn = connection.lock().unwrap();
    prox::update_user(&conn, User{id: user.0.id, name: user.0.name, age: user.0.age});
    JSON(json!({"status": "updated"}))
}

#[delete("/api/users/<idx>")]
fn delete(connection: State<Mutex<PgConnection>>, idx: usize) -> JSON<Value> {
    let conn = connection.lock().unwrap();
    match prox::delete_user(&conn, idx as i32) {
        Ok(_) => { JSON(json!({"status": "deleted"})) },
        Err(e) => { JSON(json!({"status": format!("{}", e)})) }
    }
}

#[delete("/api/users")]
fn delete_all(connection: State<Mutex<PgConnection>>) -> JSON<Value> {
    let conn = connection.lock().unwrap();
    match prox::delete_all_user(&conn) {
        Ok(_) => { JSON(json!({"status": "deleted"})) },
        Err(e) => { JSON(json!({"status": format!("{}", e)})) }
    }
}

fn main() {

    rocket::ignite().manage(Mutex::new(prox::establish_connection()))
        .mount("/", routes![index, files, show, show_all, create, update, delete, delete_all])
        .launch();
}
