use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

// Define the User struct
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
}

// Application state
struct AppState {
    users: Mutex<Vec<User>>,
}


// Handler to create a new user
#[post("/users")]
async fn create_user(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let new_user = User {
        id: (users.len() as u32) + 1,
        name: user.name.clone(),
    };
    users.push(new_user.clone());
    HttpResponse::Ok().json(new_user)
}

// Handler to get all users
#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

// Handler to get a user by ID
#[get("/users/{id}")]
async fn get_user(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_id = path.into_inner();
    match users.iter().find(|u| u.id == user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// Handler to delete a user by ID
#[delete("/users/{id}")]
async fn delete_user(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let user_id = path.into_inner();
    let initial_len = users.len();
    users.retain(|u| u.id != user_id);
    if users.len() < initial_len {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });

    // Configure and start the server
    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE]); 
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(create_user)
            .service(get_users)
            .service(get_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}