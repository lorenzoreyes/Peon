use chrono::NaiveDateTime;
use leptos::*;
use serde::{Deserialize, Serialize};

mod models;
use models::Table;

pub async fn fetch_tables(url: String) -> Result<String, ()> {
    let response: Table = reqwasm::http::Request::get(&url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("Data\n{:#?}", &response);

    let mut data: Vec<String> = Vec::new();
    // Headers of the Table
    data.push(format!(r#"<table><tr><th>Fecha</th><th>Oficial</th><th>Solidario</th><th>Cable</th><th>Monetario</th><th>Fundamental</th><th>Bonos</th></tr>"#));

    // Iterate struct into an HTML::table::values format
    for i in response.data.iter() {
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
    let mut data = data.join("\n").replace(".", ",");
    data.push_str("</table>");

    std::fs::write("table.html", &data).expect("Unable to write file");
    println!("Table was writed succesfully!");

    Ok(data)
}

#[component]
pub async fn fetch_example(cx: Scope) -> impl IntoView {
    let url = String::from("http://127.0.0.1:5000/market");

    let result = fetch_tables(url).await.unwrap();

    view! { cx,
        <Await
            future=|cx| result
            bind:data
        >
            <table>{*data}</table>
        </Await>
    }
}

