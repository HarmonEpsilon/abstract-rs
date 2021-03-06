use database::schema::users;

/* Users */
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub user: String,
    pub pass: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub user: &'a str,
    pub pass: &'a str,
    pub email: &'a str,
}