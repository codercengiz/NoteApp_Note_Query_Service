use crate::{
    models::{
        BasicInfoOfNoteChangedEventModel, NoteCreatedEventModel, ParentOfNoteChangedEventModel,
    },
    settings::MongodbSettings,
};
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Database};

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
        collection.insert_one(document, None).await;
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
            .map_err(|err| error!("{}", err))
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
            .map_err(|err| error!("{}", err))
            .ok();
    }
}
