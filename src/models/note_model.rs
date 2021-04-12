use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};



#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct NoteModel {
//    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//pub id: Option<mongodb::bson::oid::ObjectId>,
pub _id: String,
pub pid:String,
pub user_id:String,
#[serde(with = "ts_milliseconds")]
pub create_date:DateTime<Utc>,
pub text:String,
pub title:String,
pub image:String,          
pub file:String,
}