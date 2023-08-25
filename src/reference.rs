use dioxus::prelude::*;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use reqwest::Client;

mod models;
use models::Table;

#[tokio::main]
async fn main () {
    let call = call();
    println!("Resposne\t{}",call.await.unwrap());
}


async fn call() -> Result<String,reqwest::Error> {
    /// Make Request of Python Function and assign Table Struct
    let response: Table = reqwest::Client::new()
        .get("http://127.0.0.1:5000/market")
        .send()
        .await?
        .json()
        .await?;
    
    /// Data pass it to a Vector to generate proper HTML
    let mut data: Vec<String> = Vec::new();
    data.push(format!(r#"<table><tr><th>Fecha</th><th>Oficial</th><th>Solidario</th><th>Cable</th><th>Monetario</th><th>Fundamental</th><th>Bonos</th></tr>"#));
    
    /// Iterate struct to format to HTML
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
    let mut data = data.join("").replace(".",",");
    data.push_str("</table>");


    //println!("{:#?}",data);

    Ok(data) 
}


/*
fn main() {
    request();

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        section { class: "app",
            style { include_str!("../index.css") }
            h1 { 
                 h1 { "Valor X.",
                 b { class: "type",
                    h1 { "Craft the future by providing certainty" } 
                    }                    
                }
            }
            section { class: "subtitle",
                       h3 { "More than just financials doers, freedom makers." }
            }
            p { "X is an uncertainty when you don't know what you are doing, knowing it makes the whole difference." }
        }
    ))
}*/

