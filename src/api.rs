use leptos::{Scope, Serializable};
use serde::{Deserialize, Serialize};

pub fn endpoints(path: &str) -> String {
    format!("http://127.0.0.1:5000/market")
}


// fetch call function that gather the info as generic returning ok() instance
#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Table {
    pub schema: Schema,
    pub data: Vec<Datum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Datum {
    #[serde(rename = "index")]
    pub index: String,
    pub oficial: f64,
    pub solidario: f64,
    pub cable: f64,
    #[serde(rename = "Fx-Fundamental")]
    pub fx_fundamental: f64,
    #[serde(rename = "AL30")]
    pub bonds: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Schema {
    pub fields: Vec<Field>,
    #[serde(rename = "primaryKey")]
    pub primary_key: Vec<String>,
    pub pandas_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}
