use rexie::{Index, KeyRange, ObjectStore, Result, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::wasm_bindgen::JsValue;

pub async fn build_comment_database() -> Result<Rexie> {
  let rexie = Rexie::builder("cache")
    .version(1)
    .add_object_store(
      ObjectStore::new("comment").key_path("post_id"), // .auto_increment(true)
                                                       // .key_path("comment")
                                                       // .auto_increment(false)
                                                       // .add_index(Index::new("post_id", "post_id").unique(false)),
    )
    .build()
    .await?;
  // Ok(rexie)
  // let rexie = Rexie::builder("cache")
  //   .version(1)
  //   .add_object_store(
  //     ObjectStore::new("comment3")
  //       .key_path("id")
  //       .auto_increment(true)
  //       .add_index(Index::new("email", "email").unique(true)),
  //   )
  //   .build()
  //   .await?;
  Ok(rexie)
}

// pub async fn add_comment(rexie: &Rexie, post_id: i32, comment_id: i32, show: bool) -> Result<u32> {
//   let transaction = rexie.transaction(&["comment"], TransactionMode::ReadWrite)?;
//   let comments = transaction.store("comment")?;
//   let comment_meta = serde_json::json!({
//       "post_id": post_id,
//       "comment": comment_id,
//       "show": show,
//   });
//   let comment_meta_value = serde_wasm_bindgen::to_value(&comment_meta).unwrap();
//   let comment_id = comments.add(&comment_meta_value, None).await?;
//   transaction.done().await?;
//   Ok(num_traits::cast(comment_id.as_f64().unwrap()).unwrap())
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentRquest {
  pub post_id: i32,
  pub comment_id: Vec<i32>,
  // show: bool,
}

pub async fn add_array(
  rexie: &Rexie,
  post_id: i32,
  comment_id: Vec<i32>,
  // show: bool,
) -> Result<i32> {
  let transaction = rexie.transaction(&["comment"], TransactionMode::ReadWrite)?;
  let comments = transaction.store("comment")?;
  let cr = CommentRquest {
    post_id,
    comment_id,
    // show,
  };
  // let comment_meta = serde_json::json!({
  //     "post_id": post_id,
  //     "comment": comment_id,
  //     "show": show,
  // });
  // let comment_meta_value = serde_wasm_bindgen::to_value(&comment_meta).unwrap();
  let comment_meta_value = serde_wasm_bindgen::to_value(&cr).unwrap();
  let comment_id = comments.put(&comment_meta_value, None).await?;
  transaction.done().await?;
  Ok(serde_wasm_bindgen::from_value(comment_id).unwrap())
  // Ok(num_traits::cast(comment_id.as_f64().unwrap()).unwrap())
}

pub async fn get_array(rexie: &Rexie, id: i32) -> Result<Option<CommentRquest>> {
  let transaction = rexie.transaction(&["comment"], TransactionMode::ReadOnly)?;
  let employees = transaction.store("comment")?;
  if let Some(employee) = employees.get(id.into()).await? {
    let employee = serde_wasm_bindgen::from_value(employee).unwrap();
    Ok(employee)
  } else {
    Ok(Some(CommentRquest {
      post_id: id,
      comment_id: vec![],
    }))
  }
}

pub async fn get_comments(rexie: &Rexie, post_id: i32) -> Result<Vec<JsValue>> {
  let transaction = rexie.transaction(&["comment"], TransactionMode::ReadOnly)?;
  let comments = transaction.store("comment")?;
  let post_id_value = serde_wasm_bindgen::to_value(&post_id).unwrap();
  let range = KeyRange::only(&post_id_value)?;
  let post_id_index = comments.index("post_id")?;
  let values = post_id_index.get_all(Some(range), None).await?;
  Ok(values)
  // Ok(vec![])
  // keys.iter().map(|v| comments.)
  // post_id_index.
  // let comment: Option<serde_json::Value> = serde_wasm_bindgen::from_value(comment_value).unwrap();
  // Ok(comment)
  // Ok(num_traits::cast(post_id_value.as_f64().unwrap()).unwrap())
}

pub async fn build_database() -> Result<Rexie> {
  let rexie = Rexie::builder("test")
    .version(1)
    .add_object_store(
      ObjectStore::new("employees")
        .key_path("id")
        .auto_increment(true)
        .add_index(Index::new("email", "email").unique(true)),
    )
    .build()
    .await?;
  Ok(rexie)
}

pub async fn add_employee(rexie: &Rexie, name: &str, email: &str) -> Result<u32> {
  let transaction = rexie.transaction(&["employees"], TransactionMode::ReadWrite)?;
  let employees = transaction.store("employees")?;
  let employee = serde_json::json!({
      "name": name,
      "email": email,
  });
  let employee = serde_wasm_bindgen::to_value(&employee).unwrap();
  let employee_id = employees.add(&employee, None).await?;
  transaction.done().await?;
  Ok(num_traits::cast(employee_id.as_f64().unwrap()).unwrap())
}

pub async fn get_employee(rexie: &Rexie, id: u32) -> Result<Option<Value>> {
  let transaction = rexie.transaction(&["employees"], TransactionMode::ReadOnly)?;
  let employees = transaction.store("employees")?;
  let employee = employees.get(id.into()).await?.unwrap();
  let employee: Option<serde_json::Value> = serde_wasm_bindgen::from_value(employee).unwrap();
  Ok(employee)
}
