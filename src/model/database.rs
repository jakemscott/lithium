#[derive(Serialize, Deserialize, Queryable)]
pub struct Links {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub visible: bool
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub salt: String,
    pub password: String,
    pub admin: bool
}
