use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::get;
use rocket::launch;
use rocket::routes;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Josh I really work hard to be like you!"
}

#[get("/json")]
fn json_example() -> Json<Message> {
    Json(Message {
        message: "Hello, JSON!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, json_example])
        .launch()
        .await;
}
