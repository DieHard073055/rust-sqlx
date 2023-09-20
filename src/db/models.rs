#[derive(Debug, sqlx::FromRow)]
pub struct Contact {
    pub contact_id: i32,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub birthday: Option<String>,
}
