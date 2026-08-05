pub struct Vault {
    pub id: i64,
    pub name: String,
    pub desc: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct Entry {
    pub id: i64,
    pub access_key: String,
    pub access_token: String,
    pub token_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
