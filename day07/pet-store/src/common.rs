use rocket::{request::{FromRequest, Outcome}, Request};
use serde::Serialize;
use uuid::Uuid;
use rocket_db_pools::{Database};


#[derive(Clone)]
pub struct RequestId(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestId {
  type Error = ();
  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let request_id = Uuid::new_v4().to_string();
    Outcome::Success(request.local_cache(|| RequestId(request_id)).clone())
  }
}

#[derive(Database)]
#[database("pet_store")]
pub struct DbClient(mongodb::Client);


#[derive(Serialize, Debug, Clone)]
pub enum AppErrorCode {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
    ServerError
}
#[derive(Serialize, Debug, Clone)]
pub struct AppError {
  pub code: AppErrorCode,
  pub message: String,
}

impl AppError {
  pub fn bad_request(msg: &str) -> Self {
    Self {
      code: AppErrorCode::BadRequest,
      message: msg.to_owned()
    }
  }

  pub fn forbidden() -> Self {
    Self {
      code: AppErrorCode::Forbidden,
      message: String::from("forbidden")
    }
  }

  pub fn not_found(msg: &str) -> Self {
    Self {
      code: AppErrorCode::NotFound,
      message: msg.to_owned()
    }
  }

  pub fn conflict(msg: &str) -> Self {
    Self {
      code: AppErrorCode::Conflict,
      message: msg.to_owned()
    }
  }

  pub fn server_error() -> Self {
    Self {
      code: AppErrorCode::ServerError,
      message: String::from("Server Error")
    }
  }
}


impl From<mongodb::error::Error> for AppError {
  fn from(value: mongodb::error::Error) -> Self {
    log::error!("mongodb error: {}", &value);
    Self::server_error()
  }
}


pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Debug, Clone)]
pub struct IdResponse {
  pub id: String
}
#[derive(Serialize, Debug, Clone)]
pub struct PagedResponse<T> {
  pub total: u64,
  pub items: Vec<T>
}
#[derive(Serialize, Debug, Clone)]
pub struct EntityResponse<T> {
  pub data: Option<T>
}