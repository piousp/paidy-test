use actix_web::{web, guard, App, HttpResponse, HttpRequest, HttpServer, Responder, middleware};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use super::database;
use super::item;

pub fn init(port: String) {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let bind_ip = ["127.0.0.1", &port].join(":");
    println!("Server will run on {}", &bind_ip);
    let restaurant_mutex = Arc::new(Mutex::new(database::Restaurant::new()));
    let restaurant_data = web::Data::new(restaurant_mutex);
    let temp = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .register_data(restaurant_data.clone())
            .service(
                web::resource("/")
                .route(web::get().to(welcome))
            )
            .service(
                web::resource("/table/{table}")
                .route(
                    web::route()
                    .guard(guard::Post())
                    .to(insert)
                )
                .route(
                    web::route()
                    .guard(guard::Get())
                    .to(items_table)
                )
            )
            .service(
                web::resource("/table/{table}/item/{item_id}")
                .route(web::put().to(update))
                .route(web::delete().to(delete))
            )
        }
    )
    .bind(bind_ip)
    .unwrap()
    .run()
    .unwrap();
    temp
}


fn insert(req: HttpRequest,
    item: web::Json<item::ItemPayload>,
    data: web::Data<Arc<Mutex<database::Restaurant>>>) -> impl Responder {
    let table: u8 = req.match_info().get("table").unwrap().parse::<u8>().unwrap();
    match data.lock().unwrap().add_items(table, vec![item::Item::from_payload(item.0)]) {
        Ok(database::Action::Inserted) => HttpResponse::Ok().body("Inserted"),
        Err(err) => HttpResponse::InternalServerError().body(err),
        _ => HttpResponse::InternalServerError().body("This shouldn't happen")
    }
}

fn update(req: HttpRequest,
    item: web::Json<item::ItemPayload>,
    data: web::Data<Arc<Mutex<database::Restaurant>>>) -> impl Responder {
    let table: u8 = req.match_info().get("table").unwrap().parse::<u8>().unwrap();
    let item_id: Uuid = Uuid::parse_str(req.match_info().get("item_id").unwrap()).unwrap();

    match data.lock().unwrap().update_item(table, item_id, item::Item::from_payload(item.0)) {
        Ok(database::Action::Updated) => HttpResponse::Ok().body("Updated"),
        Err(err) => HttpResponse::InternalServerError().body(err),
        _ => HttpResponse::InternalServerError().body("This shouldn't happen")
    }
}

fn delete(req: HttpRequest,
    data: web::Data<Arc<Mutex<database::Restaurant>>>) -> impl Responder {
    let table: u8 = req.match_info().get("table").unwrap().parse::<u8>().unwrap();
    let item_id: Uuid = Uuid::parse_str(req.match_info().get("item_id").unwrap()).unwrap();

    match data.lock().unwrap().remove_item(table, item_id) {
        Ok(database::Action::Deleted) => HttpResponse::Ok().body("Deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err),
        _ => HttpResponse::InternalServerError().body("This shouldn't happen")
    }
}

fn items_table(req: HttpRequest, data: web::Data<Arc<Mutex<database::Restaurant>>>) -> impl Responder {
    let table: u8 = req.match_info().get("table").unwrap().parse::<u8>().unwrap();
    match data.lock().unwrap().items_from_table(table) {
        Ok(database::Action::Data(items)) => {
            let i = items;
            HttpResponse::Ok().json(i)
        },
        _ => HttpResponse::Ok().body("Table is empty")
    }
}

fn welcome() -> impl Responder {
    HttpResponse::Ok().body("This is the server / GET. You should use the RESTful functions POST/{table}, PUT/{table}, DELETE/{table} and GET/{table}")
}
