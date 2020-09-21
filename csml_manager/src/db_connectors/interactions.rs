use crate::{Client, ConversationInfo, Database, ManagerError};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "http")]
use crate::db_connectors::{is_httpdb, http as http_connector};
#[cfg(feature = "dynamo")]
use crate::db_connectors::{is_dynamodb, dynamodb as dynamodb_connector};

pub fn init_interaction(
    event: serde_json::Value,
    client: &Client,
    db: &mut Database,
) -> Result<String, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::interactions::init_interaction(event, client, db);
    }

    #[cfg(feature = "http")]
    if is_httpdb() {
        let db = http_connector::get_db(db)?;
        return http_connector::interactions::init_interaction(event, client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::interactions::init_interaction(event, client, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn update_interaction(data: &mut ConversationInfo, success: bool) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(&data.db)?;
        return mongodb_connector::interactions::update_interaction(&data.interaction_id, success, &data.client, db);
    }

    #[cfg(feature = "http")]
    if is_httpdb() {
        let db = http_connector::get_db(&data.db)?;
        return http_connector::interactions::update_interaction(&data.interaction_id, success, &data.client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(&mut data.db)?;
        return dynamodb_connector::interactions::update_interaction(&data.interaction_id, success, &data.client, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}