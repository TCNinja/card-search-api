mod model;

use actix_web::{get, App, HttpResponse, HttpServer};
use model::{Card, Game};

/// Gets the cards with the supplied id.
#[get("/search/cards")]
async fn query_cards() -> HttpResponse {
    HttpResponse::Ok().json(Card {
        game: Game::MagicTheGathering,
        name: "Some Card",
        description: "Some description",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(query_cards))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
