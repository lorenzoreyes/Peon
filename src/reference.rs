use chrono::prelude::*;
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use reqwest::Client;

mod models;
use models::Table;

#[tokio::main]
async fn main() {
    let call = call();
    println!("Resposne\t{}", call.await.unwrap());
}

async fn call() -> Result<String, reqwest::Error> {
    /// Make Request of Python Function and assign Table Struct
    let response: Table = reqwest::Client::new()
        .get("http://127.0.0.1:5000/market")
        .send()
        .await?
        .json()
        .await?;

    // Data pass it to a Vector to generate proper HTML
    let mut data: Vec<String> = Vec::new();
    // Add Headers
    data.push(format!(r#"<table><tr><th>Fecha</th><th>Oficial</th><th>Solidario</th><th>Cable</th><th>Monetario</th><th>Fundamental</th><th>Bonos</th></tr>"#));

    // Iterate struct to format to HTML
    for i in response.data.iter() {
        // timestamp format to Year-Month-Day
        let date = NaiveDateTime::parse_from_str(&i.index, "%Y-%m-%dT%H:%M:%S.%f");
        let date = date.expect("Cant Reformat").format("%d/%m/%Y");

        data.push(format!(r#"<tr><td>{}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>"#,
                    date,
                    i.oficial,
                    i.solidario,
                    i.cable,
                    i.monetarista,
                    i.fx_fundamental,
                    i.bonds));
    }
    let mut data = data.join("").replace(".", ",");
    data.push_str("</table>");

    Ok(data)
}
