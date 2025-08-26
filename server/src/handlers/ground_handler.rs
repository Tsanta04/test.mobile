use crate::dto::ground_dto::{CreateGroundDto, GroundDto, UpdateGroundDto};
use crate::error::AppError;
use crate::services::ground_service::GroundService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/grounds")]
pub async fn get_all_grounds(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_all(&mut conn) {
        Ok(grounds) => {
            let ground_dtos: Vec<GroundDto> = grounds.into_iter().map(GroundDto::from).collect();
            HttpResponse::Ok().json(ground_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/grounds/{id}")]
pub async fn get_ground_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_by_id(id, &mut conn) {
        Ok(ground) => HttpResponse::Ok().json(GroundDto::from(ground)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("Ground with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[post("/grounds")]
pub async fn create_ground(
    pool: web::Data<DbPool>,
    ground_dto: web::Json<CreateGroundDto>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_ground = ground_dto.into_inner().to_new_ground();

    match GroundService::create(new_ground, &mut conn) {
        Ok(ground) => HttpResponse::Created().json(GroundDto::from(ground)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/grounds/{id}")]
pub async fn update_ground(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    ground_dto: web::Json<UpdateGroundDto>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let ground_update = ground_dto.into_inner().to_ground_update();

    match GroundService::update(id, ground_update, &mut conn) {
        Ok(ground) => HttpResponse::Ok().json(GroundDto::from(ground)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("Ground with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[delete("/grounds/{id}")]
pub async fn delete_ground(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::delete(id, &mut conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().body(format!("Ground with ID {} not found", id))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/grounds/user/{user_id}")]
pub async fn get_grounds_by_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_by_user(user_id, &mut conn) {
        Ok(grounds) => {
            let ground_dtos: Vec<GroundDto> = grounds.into_iter().map(GroundDto::from).collect();
            HttpResponse::Ok().json(ground_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/grounds/culture/{culture_type_id}")]
pub async fn get_grounds_by_culture_type(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let culture_type_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_by_culture_type(culture_type_id, &mut conn) {
        Ok(grounds) => {
            let ground_dtos: Vec<GroundDto> = grounds.into_iter().map(GroundDto::from).collect();
            HttpResponse::Ok().json(ground_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/grounds/location/{location_id}")]
pub async fn get_grounds_by_location(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let location_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_by_location(location_id, &mut conn) {
        Ok(grounds) => {
            let ground_dtos: Vec<GroundDto> = grounds.into_iter().map(GroundDto::from).collect();
            HttpResponse::Ok().json(ground_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/grounds/pack/{pack_id}")]
pub async fn get_grounds_by_pack(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let pack_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match GroundService::find_by_pack(&pack_id, &mut conn) {
        Ok(grounds) => {
            let ground_dtos: Vec<GroundDto> = grounds.into_iter().map(GroundDto::from).collect();
            HttpResponse::Ok().json(ground_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_all_grounds)
            .service(get_ground_by_id)
            .service(create_ground)
            .service(update_ground)
            .service(delete_ground)
            .service(get_grounds_by_user)
            .service(get_grounds_by_culture_type)
            .service(get_grounds_by_location)
            .service(get_grounds_by_pack),
    );
}

