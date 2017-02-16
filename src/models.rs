use schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}
