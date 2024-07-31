use mongodb::bson::oid::ObjectId;
use rocket::{ fairing::AdHoc, routes, FromForm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Category {
  Cats,
  Dogs,
  Pigs,
  Lizards,
}

impl TryFrom<String> for Category {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
      match value.as_str() {
          "Cats" => Ok(Self::Cats),
          "Dogs" => Ok(Self::Dogs),
          "Pigs" => Ok(Self::Pigs),
          "Lizards" => Ok(Self::Lizards),
          s @ _ => Err(format!("Invalid Catrgory, {}", s))
      }
  }
}

impl TryFrom<&str> for Category {
  type Error = String;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
      Category::try_from(value.to_owned())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Status {
  Available,
  Pending,
  Sold,
}

impl TryFrom<String> for Status {
  type Error = String;
  fn try_from(value: String) -> Result<Self, Self::Error> {
      match value.as_str() {
        "Available" => Ok(Self::Available),   
        "Pending" => Ok(Self::Pending),   
        "Sold" => Ok(Self::Sold),   
        s @ _ => Err(format!("Invalid Status: {}", s))
      }
  }
}
impl TryFrom<&str> for Status {
  type Error = String;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
      Status::try_from(value.to_owned())
  }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PetDocument {
  #[serde(rename="_id")]
  pub id: ObjectId,

  pub name: String,
  pub age: u32,
  pub category: Category,
  pub status: Status
}

// impl From<PetCreateForm> for PetDocument {
//   fn from(value: PetCreateForm) -> Self {
//       Self {
//         id: ObjectId::new(),
//         name: value.name,
//         age: value.age,
//         category: value.category.try_into().unwrap(),
//         status: value.status.try_into().unwrap()
//       }
//   }
// }

impl TryFrom<PetCreateForm> for PetDocument {
  type Error = String;

  fn try_from(value: PetCreateForm) -> Result<Self, Self::Error> {
      if (&value.name.trim()).is_empty() {
        return Err("field naame is required".into())
      }

      let category = Category::try_from(value.category.as_str())?;
      let status = Status::try_from(value.status.as_str())?;

      Ok(Self {
        id: ObjectId::new(),
        name: value.name,
        age: value.age,
        category,
        status
      })
  }
}

#[derive(FromForm)]
pub struct PetQueryForm {
  #[field(default = 1)]
  pub page: u64,
  #[field(name = "itemsPerPage", default = 20)]
  pub items_per_page: u64,
  pub status: Option<String>,
  pub name: Option<String>
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PetCreateForm {
  pub name: String,
  pub age: u32,
  pub category: String,
  pub status: String,
}

pub mod controller {
    use rocket::{get, post, serde::json::Json, form::{Form, self}};
    use rocket_db_pools::Connection;

    use crate::common::{DbClient, AppResult, IdResponse, PagedResponse, EntityResponse};

    use super::{PetCreateForm, service, PetQueryForm, PetDocument};
  #[post("/", data = "<data>")]
  pub async fn create(client: Connection<DbClient>, data: Json<PetCreateForm>) -> Json<AppResult<IdResponse>> {
    // "OK".to_string()
    // let oid = service::create(&client, form.into_inner()).await;
    // oid.to_hex()

    match service::create(&client, data.into_inner()).await {
        Ok(s) => Json(Ok((IdResponse {id: s.to_hex()}))),
        Err(e) => return Json(Err(e))
    }
  }

  #[get("/?<form..>")]
  pub async fn query(client: Connection<DbClient>, form: PetQueryForm) -> Json<AppResult<PagedResponse<PetDocument>>> {
    Json(service::query(&client, form).await)
  }

  #[get("/<id>")]
  pub async fn detail(client: Connection<DbClient>, id: String) -> Json<AppResult<EntityResponse<PetDocument>>> {
    let doc = match service::get_by_id(&client, id.as_str()).await {
        Ok(r) => r,
        Err(e) => return Json(Err(e))
    };
    Json(Ok(EntityResponse {
      data: doc
    }))
  }

  // #[get("/", form = "<form>")]
  // pub async fn query(client: Connection<DbClient>, form: Form<PetQueryForm>) -> Json<AppResult<Vec<PetDocument>> {

  // }
}


pub mod service {
    use mongodb::bson::oid::ObjectId;
    use rocket_db_pools::Connection;

    use crate::common::{DbClient, AppResult, AppError, PagedResponse};

    use super::{PetCreateForm, PetDocument, dao};

  pub async fn create(client: &Connection<DbClient>, form: PetCreateForm) -> AppResult<ObjectId> {
    // let doc: PetDocument = form.into();
    // doc.id
    // dao::insert_one(client, &doc).await

    let doc = match PetDocument::try_from(form) {
      Ok(d) => d,
      Err(s) => return Err(AppError::bad_request(s.as_str()))
    };
    dao::insert_one(client, &doc).await
  }

  pub async fn query(client: &Connection<DbClient>, form: super::PetQueryForm) -> AppResult<PagedResponse<PetDocument>> {
    let total = dao::count(client, &form).await?;
    let items = dao::find(client, &form).await?;
    Ok(PagedResponse {
      total,
      items
    })

  }

  pub async fn get_by_id(client: &Connection<DbClient>, id: &str) -> AppResult<Option<PetDocument>> {
    let oid = match ObjectId::parse_str(id) {
      Ok(v) => v,
      Err(_) => return Err(AppError::bad_request(format!("invalid id {}", id).as_str()))
    };
    dao::find_by_id(client, oid).await
  }
}

pub mod dao {
    use mongodb::{bson::{oid::ObjectId, Document, doc}, options::FindOptions};
    use rocket_db_pools::Connection;

    use crate::common::{DbClient, AppResult};

    use super::{PetDocument, PetQueryForm, Status};
    use rocket::futures::TryStreamExt;
  pub async fn insert_one(client: &Connection<DbClient>, doc: &PetDocument) -> AppResult<ObjectId> {
    let col = client.default_database().unwrap().collection::<PetDocument>("pets");
    let ret = col.insert_one(doc, None).await?;
    // ret.unwrap().inserted_id.as_object_id().unwrap()
    Ok(ret.inserted_id.as_object_id().unwrap())
  }
  
  fn build_filter(form: &PetQueryForm) -> Option<Document> {
    let status = match &form.status {
        Some(s) => match Status::try_from(s.as_str()) {
          Ok(st) => Some(st),
          Err(_) => None
        },
        None => None
    };

    let name  = form.name.clone();

    if status.is_none() && name.is_none() {
      return None;
    }

    let mut doc = doc! { };
    if status.is_some() {
      doc.insert("status", &form.status);
    }

    if name.is_some() {
      doc.insert("name", doc! {
        "$regex": name.unwrap()
      });
    }

    Some(doc)
  }


  pub async fn count(client: &Connection<DbClient>, form: &PetQueryForm) -> AppResult<u64> {
    let col = client.default_database()
    .unwrap()
    .collection::<PetDocument>("pets");
    let filter = build_filter(form);
    Ok(col.count_documents(filter, None).await?)
  }

  pub async fn find(client: &Connection<DbClient>, form: &PetQueryForm) -> AppResult<Vec<PetDocument>> {
    let col = client.default_database()
    .unwrap()
    .collection::<PetDocument>("pets");
    let filter = build_filter(form);
    let options = FindOptions::builder()
      .sort(doc! {
        "name": 1
      })
      .skip((form.page - 1) * form.items_per_page)
      .limit(form.items_per_page as i64)
      .build();
    Ok(col.find(filter, options).await?.try_collect::<Vec<_>>().await?)
    // let r = col.find(filter, options).await?;
  }

  pub async fn find_by_id(client: &Connection<DbClient>, id: ObjectId) -> AppResult<Option<PetDocument>> {
    let col = client.default_database().unwrap().collection::<PetDocument>("pets");
    Ok(col.find_one(doc! {
      "_id": id
    }, None).await?)
  }


}


pub fn stage() -> AdHoc {
  AdHoc::on_ignite("pets", | rocket | async {
    rocket.mount("/pets", routes![controller::create, controller::query, controller::detail])
  })
}