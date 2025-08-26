use crate::dto::alert_dto::{AlertDto, CreateAlertDto, UpdateAlertDto};
use crate::error::AppError;
use crate::services::alert_service::AlertService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/alerts")]
pub async fn get_all_alerts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::find_all(&mut conn) {
        Ok(alerts) => {
            let alert_dtos: Vec<AlertDto> = alerts.into_iter().map(AlertDto::from).collect();
            HttpResponse::Ok().json(alert_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/alerts/{id}")]
pub async fn get_alert_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::find_by_id(id, &mut conn) {
        Ok(alert) => HttpResponse::Ok().json(AlertDto::from(alert)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("Alert with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[post("/alerts")]
pub async fn create_alert(
    pool: web::Data<DbPool>,
    alert_dto: web::Json<CreateAlertDto>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_alert = alert_dto.into_inner().to_new_alert();

    match AlertService::create(new_alert, &mut conn) {
        Ok(alert) => HttpResponse::Created().json(AlertDto::from(alert)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/alerts/{id}")]
pub async fn update_alert(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    alert_dto: web::Json<UpdateAlertDto>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let alert_update = alert_dto.into_inner().to_alert_update();

    match AlertService::update(id, alert_update, &mut conn) {
        Ok(alert) => HttpResponse::Ok().json(AlertDto::from(alert)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("Alert with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[delete("/alerts/{id}")]
pub async fn delete_alert(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::delete(id, &mut conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().body(format!("Alert with ID {} not found", id))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/alerts/state/{state_id}")]
pub async fn get_alerts_by_state(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let state_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::find_by_state(state_id, &mut conn) {
        Ok(alerts) => {
            let alert_dtos: Vec<AlertDto> = alerts.into_iter().map(AlertDto::from).collect();
            HttpResponse::Ok().json(alert_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/alerts/type/{alert_type}")]
pub async fn get_alerts_by_type(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let alert_type = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::find_by_type(&alert_type, &mut conn) {
        Ok(alerts) => {
            let alert_dtos: Vec<AlertDto> = alerts.into_iter().map(AlertDto::from).collect();
            HttpResponse::Ok().json(alert_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/alerts/unseen")]
pub async fn get_unseen_alerts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::find_unseen(&mut conn) {
        Ok(alerts) => {
            let alert_dtos: Vec<AlertDto> = alerts.into_iter().map(AlertDto::from).collect();
            HttpResponse::Ok().json(alert_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/alerts/{id}/seen")]
pub async fn mark_alert_as_seen(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match AlertService::mark_as_seen(id, &mut conn) {
        Ok(alert) => HttpResponse::Ok().json(AlertDto::from(alert)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().body(format!("Alert with ID {} not found", id))
            }
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_all_alerts)
            .service(get_alert_by_id)
            .service(create_alert)
            .service(update_alert)
            .service(delete_alert)
            .service(get_alerts_by_state)
            .service(get_alerts_by_type)
            .service(get_unseen_alerts)
            .service(mark_alert_as_seen),
    );
}

