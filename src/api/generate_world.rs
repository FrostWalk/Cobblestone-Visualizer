use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct WorldGenData {
    size: usize,
    seed: u64,
    wait: u64,
}

#[derive(Serialize)]
struct GenResponse {
    success: bool,
    error: String,
}

/*#[post("/generateWorld")]
async fn post_ticket(req: web::Json<WorldGenData>) -> impl Responder {
    gene

    let response = serde_json::to_string(&new_ticket).unwrap();

    tickets.push(new_ticket);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}*/