use mongodb::{options::ClientOptions, Client};

use crate::settings::MongodbSettings;

pub struct MongodbService {
    mongo_client: mongodb::Client,
}
impl MongodbService {
    pub(crate) async fn init(settings: MongodbSettings) -> Self {
        let mut client_options = ClientOptions::parse(&settings.connectionstring)
            .await
            .unwrap();

        let mongo_client = Client::with_options(client_options).unwrap();

        MongodbService { mongo_client }
    }
}
