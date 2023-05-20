mod database;

use database::*;
use serde::{Deserialize, Serialize};

use actix_web::{
    post,
    web,
    App,
    error,
    Result,
    HttpResponse,
    HttpServer,
    Responder
};

#[derive(Deserialize)]
struct WordFormData {
    word: String
}

#[derive(Serialize)]
struct DefaultResponse {
    message: String
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn get_word(form: web::Form<WordFormData>) -> Result<impl Responder> {
    let results = get(&form.word);
    match results {
        Ok(value) => {
            Ok(web::Json(value))
        },
        Err(_) => {
            Err(error::ErrorInternalServerError("DB error"))
        }
    }
}

async fn add_word (form: web::Form<NewVocaItem>) -> Result<impl Responder> {
    let item = form.into_inner();
    match add(item) {
        Ok(res) => {
            Ok(web::Json(DefaultResponse {
                message: format!("Add {} record(s)", res)
            }))
        },
        Err(_) => {
            Err(error::ErrorInternalServerError("DB error"))
        }
    }
}

async fn update_word (form: web::Form<VocaItem>) -> Result<impl Responder> {
    let item = form.into_inner();
    match update(item) {
        Ok(res) => {
            Ok(web::Json(DefaultResponse {
                message: format!("Patch {} record(s)", res)
            }))
        },
        Err(_) => {
            Err(error::ErrorInternalServerError("DB error"))
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(web::scope("/word")
                .route("", web::get().to(get_word))
                .route("", web::post().to(add_word))
                .route("", web::patch().to(update_word))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
