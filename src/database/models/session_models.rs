table! {
    sessions {
        id -> VarChar,
        user_id -> Integer,
    }
}

/* User Sessions */
#[derive(Queryable)]
pub struct Session {
    pub id: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="sessions"]
pub struct NewSession {
    pub id: String,
    pub user_id: i32,
}