use crate::dto::state_dto::{CreateStateDto, DateRangeDto, StateDto, UpdateStateDto};
use crate::error::AppError;
use crate::services::state_service::StateService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/states")]
pub async fn get_all_states(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::find_all(&mut conn) {
        Ok(states) => {
            let state_dtos: Vec<StateDto> = states.into_iter().map(StateDto::from).collect();
            HttpResponse::Ok().json(state_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/states/{id}")]
pub async fn get_state_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::find_by_id(id, &mut conn) {
        Ok(state) => HttpResponse::Ok().json(StateDto::from(state)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("State with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[post("/states")]
pub async fn create_state(
    pool: web::Data<DbPool>,
    state_dto: web::Json<CreateStateDto>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_state = state_dto.into_inner().to_new_state();

    match StateService::create(new_state, &mut conn) {
        Ok(state) => HttpResponse::Created().json(StateDto::from(state)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/states/{id}")]
pub async fn update_state(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    state_dto: web::Json<UpdateStateDto>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let state_update = state_dto.into_inner().to_state_update();

    match StateService::update(id, state_update, &mut conn) {
        Ok(state) => HttpResponse::Ok().json(StateDto::from(state)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("State with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[delete("/states/{id}")]
pub async fn delete_state(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::delete(id, &mut conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().body(format!("State with ID {} not found", id))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/states/pack/{pack_id}")]
pub async fn get_states_by_pack(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let pack_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::find_by_pack(&pack_id, &mut conn) {
        Ok(states) => {
            let state_dtos: Vec<StateDto> = states.into_iter().map(StateDto::from).collect();
            HttpResponse::Ok().json(state_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/states/pack/{pack_id}/latest")]
pub async fn get_latest_state_by_pack(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let pack_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::find_latest_by_pack(&pack_id, &mut conn) {
        Ok(state) => HttpResponse::Ok().json(StateDto::from(state)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("No states found for pack ID {}", pack_id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[post("/states/pack/{pack_id}/date-range")]
pub async fn get_states_by_date_range(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    date_range: web::Json<DateRangeDto>,
) -> impl Responder {
    let pack_id = path.into_inner();
    let date_range = date_range.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match StateService::find_by_date_range(
        &pack_id,
        date_range.start_date,
        date_range.end_date,
        &mut conn,
    ) {
        Ok(states) => {
            let state_dtos: Vec<StateDto> = states.into_iter().map(StateDto::from).collect();
            HttpResponse::Ok().json(state_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_all_states)
            .service(get_state_by_id)
            .service(create_state)
            .service(update_state)
            .service(delete_state)
            .service(get_states_by_pack)
            .service(get_latest_state_by_pack)
            .service(get_states_by_date_range),
    );
}

