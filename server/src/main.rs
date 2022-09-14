use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Result};
use actix_rt;
use std::sync::{Mutex};
use std::time::{Duration};
use tokio::time;
use std::collections::{HashMap};

mod engine;
pub use crate::engine::game_controller::{GameState}; 



#[get("/snake")]
async fn greet(data: web::Data<Mutex<GameState>>) -> impl Responder {
    let state = data.lock().unwrap().clone();
    let j = serde_json::to_string(&state).unwrap();
    format!("{}", j)
}

#[post("/snake/:{direction}")]
async fn direction(direction: web::Path<String>, data: web::Data<Mutex<GameState>>) -> impl Responder {
    let mut game_state = data.lock().unwrap();
    *game_state.user_input.get_mut(&direction.to_string()).unwrap() += 1;
    HttpResponse::Ok()
}

async fn index(data: web::Data<Mutex<GameState>>) -> Result<HttpResponse> {
    let game_state = data.lock().unwrap().clone();

    if game_state.running == false {
        actix_rt::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut game_state = data.lock().unwrap();
                game_state.running = true;

                game_state.update_direction();

                if let Err(_) = game_state.move_snake() {
                    game_state.reset();
                }
                game_state.update_fruits();

                clear_input(&mut game_state.user_input);
            }
        });
    }

    Ok(HttpResponse::Ok().body("OK"))
}


fn clear_input(inputs: &mut HashMap<String, u32>){
    *inputs.get_mut("left").unwrap() = 0;
    *inputs.get_mut("right").unwrap() = 0;
    *inputs.get_mut("up").unwrap() = 0;
    *inputs.get_mut("down").unwrap() = 0;
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(Mutex::new(GameState::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .route("/", web::get().to(index))
            .service(greet)
            .service(direction)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}