mod model;

use actix_web::{get, App, HttpResponse, HttpServer};
use model::{Card, Game, QueryResult};
use uuid::uuid;

/// Gets the cards with the supplied id.
#[get("/search/cards")]
async fn query_cards() -> HttpResponse {
    HttpResponse::Ok().json(QueryResult {
        results: vec![Card {
            id: uuid!("d4d8c9f9-31ed-53ed-ab67-eba86e2198fe"),
            game: Game::MagicTheGathering,
            name: "Black Lotus",
            description: "{T}, Sacrifice Black Lotus: Add three mana of any one color.",
        }],
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(query_cards))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
