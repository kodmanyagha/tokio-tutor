use std::path::Path;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use log::info;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    rust_log: String,
    database_url: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found.");
    pretty_env_logger::init();

    let config = envy::from_env::<Config>().unwrap();
    println!("DATABASE URL: {}", config.database_url);
    println!("LOG LEVEL: {}", config.rust_log);

    let host = "127.0.0.1";
    let port = "3344";
    let server_point = format!("{}:{}", host, port);

    info!("Starting server at http://{server_point}");

    let listener = tokio::net::TcpListener::bind(server_point)
        .await
        .expect("Can not listen network.");
    let router = axum::Router::new();
    let router = router.merge(Router::new().route("/", get(hello_world)));

    let axum_result = axum::serve(listener, router).await.map_err(|err| {
        println!("{:?}", err);
        "Error occured when running server."
    });

    match axum_result {
        Ok(_) => todo!(),
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

async fn hello_world() -> Html<String> {
    let x = liquid::partials::InMemorySource::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(liquid::partials::EagerCompiler::new(
            liquid::partials::InMemorySource::new(),
        ))
        .build()
        .unwrap()
        .parse_file(Path::new("./templates/dark_blog_1/index.tpl.html"))
        .unwrap();

    let globals = liquid::object!({
        "num": 334455
    });

    let output = template.render(&globals).unwrap();

    Html(output)
}

async fn main_page() -> Html<String> {
    Html("This is <strong>main page</strong>...".to_string())
}
