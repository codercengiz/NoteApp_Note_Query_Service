use clap::{App, ArgMatches};

use std::fmt::Debug;
use std::net::SocketAddr;

pub(crate) struct Settings {
    pub web_server_settings: WebServerSettings,
    pub kafka_settings: KafkaSettings,
    pub mongodb_settings: MongodbSettings,
}

impl Settings {
    pub(crate) fn init() -> Settings {
        let yaml = load_yaml!("app_settings.yaml");
        let matches = App::from(yaml).get_matches();
        Settings {
            web_server_settings: Self::get_web_api_settings(&matches),
            kafka_settings: Self::get_kafka_settings(&matches),
            mongodb_settings: Self::get_mongodb_settings(&matches),
        }
    }

    fn get_web_api_settings(matches: &ArgMatches) -> WebServerSettings {
        let domain = matches
            .value_of("domain")
            .expect("Invalid Domain")
            .to_string();
        let port = matches.value_of("port").expect("Invalid Port").to_string();
        let addr_string = format!("{}:{}", domain, port);
        let socket_addr = addr_string.parse().expect("Invalid Socket Address");

        WebServerSettings { socket_addr }
    }

    fn get_kafka_settings(matches: &ArgMatches) -> KafkaSettings {
        KafkaSettings {
            broker: matches
                .value_of("kafka-brokers-addr")
                .expect("Invalid Brokers")
                .to_string(),

            consumer_topics: matches
                .values_of("kafka-consumer-topics")
                .expect("Invalid Topics")
                .map(|x| x.to_string())
                .collect(),
        }
    }

    fn get_mongodb_settings(matches: &ArgMatches) -> MongodbSettings {
        let host = matches
            .value_of("mongodb-host")
            .expect("Invalid mongodb host")
            .to_string();

        let username = matches
            .value_of("mongodb-username")
            .expect("Invalid Mongodb Username")
            .to_string();
        let password = matches
            .value_of("mongodb-password")
            .expect("Invalid Mongodb Password")
            .to_string();
        MongodbSettings {
            host: host.clone(),
            username: username.clone(),
            password: password.clone(),
            collection: matches
                .value_of("mongodb-collection")
                .expect("Invalid Mongodb Password")
                .to_string(),
            connectionstring: format!("mongodb://{}:{}@{}", username, password, host),
        }
    }
}

#[derive(Debug)]
pub(crate) struct WebServerSettings {
    pub(crate) socket_addr: SocketAddr,
}

#[derive(Debug)]
pub(crate) struct KafkaSettings {
    pub(crate) broker: String,
    pub(crate) consumer_topics: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct MongodbSettings {
    pub(crate) host: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) collection: String,
    pub(crate) connectionstring: String,
}
