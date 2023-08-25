use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub schema: Schema,
    pub data: Vec<Datum>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Datum {
    #[serde(rename = "index")]
    pub index: String,
    pub oficial: f64,
    pub solidario: f64,
    pub cable: f64,
    #[serde(rename = "Fx-Fundamental")]
    pub fx_fundamental: f64,
    pub monetarista: f64,
    #[serde(rename = "AL30")]
    pub bonds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<Field>,
    #[serde(rename = "primaryKey")]
    pub primary_key: Vec<String>,
    pub pandas_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

