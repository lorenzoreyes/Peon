use leptos::{*};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::NaiveDateTime;

mod models;
use models::Table;


#[derive(Error, Clone, Debug)]
pub enum TableError {
    #[error("Please request more than zero Table.")]
    NonZeroTable,
}

pub async fn fetch_tables(url: String) -> Result<String, reqwasm::Error> {
    let response: Table = reqwasm::http::Request::get(&url)        
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("Data\n{:#?}",&response);

    let mut data: Vec<String> = Vec::new();
    data.push(format!(r#"<table><tr><th>Fecha</th><th>Oficial</th><th>Solidario</th><th>Cable</th><th>Monetario</th><th>Fundamental</th><th>Bonos</th></tr>"#));
    
    // Iterate struct into an HTML format
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
    let mut data = data.join("\n").replace(".",",");
    data.push_str("</table>");
    
    std::fs::write("table.html",&data).expect("Unable to write file");
    println!("Table was writed succesfully!");
 
    Ok(data)
}

pub async fn fetch_example(cx: Scope) -> impl IntoView {
    let url = String::from("http://127.0.0.1:5000/market"); 

    let result = fetch_tables(url).await.unwrap();

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                    .collect_view(cx)
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just implement our happy path and let the framework handle the rest
    view! { cx,
        <div>
            <label>
                "Dollars value in Argentina"
                <table>{result.clone()}</table>
            </label>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { cx, <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {result.clone()}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
