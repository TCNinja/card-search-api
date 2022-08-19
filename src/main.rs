mod model;

use lambda_web::actix_web::{self, get, App, HttpResponse, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use model::{Card, Game, QueryResult};
use uuid::uuid;

/// Gets the cards with the supplied id.
#[get("/cards/search")]
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
async fn main() -> Result<(), LambdaError> {
    let factory = move || App::new().service(query_cards);

    if is_running_on_lambda() {
        run_actix_on_lambda(factory).await?;
    } else {
        HttpServer::new(factory)
            .bind(("0.0.0.0", 8080))?
            .run()
            .await?;
    }

    Ok(())
}
