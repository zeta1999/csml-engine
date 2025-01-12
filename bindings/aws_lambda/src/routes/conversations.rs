use csml_engine::{user_close_all_conversations, get_open_conversation, Client};

use lambda_runtime::error::HandlerError;

pub fn get_open(body: Client) -> Result<serde_json::Value, HandlerError> {

    let res = get_open_conversation(&body);

    match res {
        Ok(Some(conversation)) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(conversation).to_string()
            }
        )),
        Ok(None) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(null).to_string()
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(HandlerError::from(error.as_str()))
        }
    }
}

pub fn close_user_conversations(body: Client) -> Result<serde_json::Value, HandlerError> {

    let res = user_close_all_conversations(body.clone());

    match res {
        Ok(()) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(null).to_string()
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(HandlerError::from(error.as_str()))
        }
    }
}
