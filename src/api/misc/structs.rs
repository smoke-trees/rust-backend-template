use serde::Serialize;

#[derive(Deserialize)]
pub struct EntityStringId {
    pub id: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Page {
    pub skip: i64,
    pub limit: i64,
}

#[derive(Serialize)]
pub struct PageData<T> {
    pub data: Vec<T>,
    pub msg: String,
}

#[derive(Serialize)]
pub struct TokenData {
    pub data: String,
    pub msg: String,
}

#[derive(Serialize)]
pub struct Wrapper<T: Serialize> {
    pub data: T,
    pub msg: String,
}
