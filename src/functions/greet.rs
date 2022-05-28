use serde::Deserialize;
use axum::extract::Query;

#[derive(Deserialize)]
pub(crate) struct Info {
    name: Option<String>,
}

pub(crate) async fn greet(info: Query<Info>) -> String {
    match info.name {
        Some(ref name) => format!("hello {}!", name),
        None => "hello world!".to_string(),
    }
}
