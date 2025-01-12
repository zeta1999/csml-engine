use crate::data::DynamoDbClient;
use crate::db_connectors::dynamodb::Interaction;
use crate::{encrypt::encrypt_data, Client, EngineError};
use rusoto_dynamodb::*;
use uuid::Uuid;

use crate::db_connectors::dynamodb::utils::*;

pub fn init_interaction(
    event: serde_json::Value,
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<String, EngineError> {
    let id = Uuid::new_v4();
    let encrypted_event = encrypt_data(&event)?;
    let interaction = Interaction::new(&id, client, &encrypted_event);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(interaction.hash.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(interaction.range.to_owned()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = PutItemInput {
        table_name: get_table_name()?,
        item: serde_dynamodb::to_hashmap(&interaction)?,
        condition_expression: Some("#hashKey <> :hashVal AND #rangeKey <> :rangeVal".to_owned()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let future = db.client.put_item(input);
    db.runtime.block_on(future)?;

    Ok(id.to_string())
}

pub fn update_interaction(
    interaction_id: &str,
    success: bool,
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let key = Interaction::get_key(client, interaction_id);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
        (String::from("#successKey"), String::from("success")),
        (String::from("#updatedAtKey"), String::from("updated_at")),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(key.hash.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(key.range.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":successVal"),
            AttributeValue {
                bool: Some(success),
                ..Default::default()
            },
        ),
        (
            String::from(":updatedAtVal"),
            AttributeValue {
                s: Some(get_date_time()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    // make sure that if the item does not already exist, it is NOT created automatically
    let condition_expr = "#hashKey = :hashVal AND #rangeKey = :rangeVal".to_string();
    let update_expr = "SET #updatedAtKey = :updatedAtVal, #successKey = :successVal".to_string();

    let input = UpdateItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&key)?,
        condition_expression: Some(condition_expr),
        update_expression: Some(update_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let future = db.client.update_item(input);
    db.runtime.block_on(future)?;

    Ok(())
}
