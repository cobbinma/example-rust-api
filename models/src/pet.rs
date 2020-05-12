#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub tag: Option<String>,
}
