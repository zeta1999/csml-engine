use actix_web::{post, web, HttpResponse};
use csml_engine::{create_bot_version};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
  bot: CsmlBot,
}


/*
* create bot version
*
*{"statusCode": 200,"body": {"version_id": String} }
*
*/
#[post("/create_bot_version")]
pub async fn handler(body: web::Json<CreateRequest>) -> HttpResponse {
  let bot = body.bot.to_owned();

  let res = thread::spawn(move || {
    create_bot_version(bot)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Created().json(serde_json::json!({"version_id": data})),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}