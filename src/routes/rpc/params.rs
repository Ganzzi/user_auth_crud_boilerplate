use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    pub id: i64,
    pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsForJustId {
    pub id: i64,
}
