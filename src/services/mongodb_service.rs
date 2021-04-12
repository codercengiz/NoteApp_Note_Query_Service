use std::convert::{TryFrom, TryInto};

use crate::{
    models::{
        BasicInfoOfNoteChangedEventModel, NoteCreatedEventModel, NoteModel,
        ParentOfNoteChangedEventModel,
    },
    settings::MongodbSettings,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use mongodb::bson::{doc, document::ValueAccessError, Document};
use mongodb::{options::ClientOptions, Client, Database};

use futures::stream::StreamExt;
use thiserror::Error;
pub struct MongodbService {
    pub mongo_client: mongodb::Client,
    pub database: Database,
}
impl MongodbService {
    pub(crate) async fn init(settings: MongodbSettings) -> Self {
        let client_options = ClientOptions::parse(&settings.connectionstring)
            .await
            .unwrap();

        let mongo_client = Client::with_options(client_options).unwrap();

        MongodbService {
            database: mongo_client.database(&settings.database),
            mongo_client: mongo_client,
        }
    }

    pub(crate) async fn add_note(&self, note_created_event_model: NoteCreatedEventModel) {
        let collection = self.database.collection::<Document>("notes");
        let document = doc! { "_id": note_created_event_model.id,
        "pid": note_created_event_model.pid,
        "user_id": note_created_event_model.user_id,
        "create_date": note_created_event_model.create_date,
        "text": note_created_event_model.text,
        "title" : note_created_event_model.title,
        "image" : note_created_event_model.image,
        "file" : note_created_event_model.file,

        };
        collection.insert_one(document, None).await.map_err( |err| error!("Error while note inserting: {:?}", err));
    }
    pub(crate) async fn change_parent_of_note(
        &self,
        parent_of_note_changed_event_model: ParentOfNoteChangedEventModel,
    ) {
        let collection = self.database.collection::<Document>("notes");
        let document = doc! { "_id": parent_of_note_changed_event_model.id
        };
        let update_document = doc! {
            "$set":{"pid":parent_of_note_changed_event_model.pid}
        };
        let _updateresult = collection
            .update_one(document, update_document, None)
            .await
            .map_err(|err|  error!("Error while parent changing: {:?}", err))
            .ok();
    }
    pub(crate) async fn change_basic_info_of_note(
        &self,
        basic_info_of_note_changed_event_model: BasicInfoOfNoteChangedEventModel,
    ) {
        let collection = self.database.collection::<Document>("notes");
        let document = doc! { "_id": basic_info_of_note_changed_event_model.id
        };
        let update_document = doc! {
            "$set":{"text":basic_info_of_note_changed_event_model.text,
             "title": basic_info_of_note_changed_event_model.title}
        };
        let _updateresult = collection
            .update_one(document, update_document, None)
            .await
            .map_err(|err| error!("Error while basic info changing: {:?}", err))
            .ok();
    }

    pub(crate) async fn get_all_notes(&self, user_id: String) -> Result<Vec<NoteModel>, MongodbServiceError> {
        let collection = self.database.collection::<Document>("notes");
        let search_query = doc! { "user_id": user_id};

        let mut cursor = collection
            .find(search_query, None)
            .await?;
            
        let mut notes:Vec<NoteModel> = vec![];    
        while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        let note = NoteModel::try_from(document)?;
                        notes.push(note);
                    }
                    Err(e) => return Err(e.into()),
                }
            }    
            
    
        Ok(notes)
    }
}

impl Clone for MongodbService {
    fn clone(&self) -> Self {
        MongodbService {
            mongo_client: self.mongo_client.clone(),
            database: self.database.clone(),
        }
    }
}
impl TryFrom<Document> for NoteModel {
    type Error = ValueAccessError;
    fn try_from(document: Document) -> Result<Self, ValueAccessError> {
        let note_id = document.get_str("_id").map(String::from)?;

        let pid = document.get_str("pid").map(String::from)?;
        let user_id = document.get_str("user_id").map(String::from)?;
        let text = document.get_str("text").map(String::from)?;
        let title = document.get_str("title").map(String::from)?;
        //let timestamp = document.get_timestamp("create_date")?;

        /*let create_date = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(timestamp.time.into(), timestamp.increment),
            Utc,
        );*/
        let image = document.get_str("image").map(String::from)?;
        let file = document.get_str("file").map(String::from)?;

        Ok(NoteModel {
            _id: note_id,
            pid,
            user_id,
            
            text,
            title,
            image,
            file,
        })
    }
}

#[derive( Error, Debug)]
pub(crate) enum MongodbServiceError {
    #[error("Mongodb error: {0}")]
    MongodbError(#[from] mongodb::error::Error),
    #[error("Valueaccess error: {0}")]
    ValueAccessError(#[from] mongodb::bson::document::ValueAccessError),
     /// Represents a failure to read from input.
    //#[error("Read error")]
    //ReadError { source: std::io::Error },
    #[error("Bson error: {0}")]
    BsonError(#[from] mongodb::bson::de::Error),
    //#[error("unknown data store error")]
    //Unknown,
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),
}
