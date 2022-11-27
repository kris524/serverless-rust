// get the curerntly configured aws chain
use aws_config::meta::region::RegionProviderChain;

// dynamodb part of the sdk
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;

//the lambda runtime that will help us tell lambda how to call our handler function
use lambda_runtime::{handler_fn, Context, Error as LambdaError};

// serde lib for deceralisation
use serde::Deserialize;
use serde_json::{json, Value};

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?; //block until the results come back
                                      // the ? will get us the caller of the main function
    Ok(()) //tells the caller things were successful
}

//set up the custom event object
#[derive(Deserialize)] //you need this to allow the handler to take this object as an argument
struct CustomEvent {
    first_name: String,
    last_name: String,
    id: i32,
}

async fn handler(event: CustomEvent, _: Context) -> Result<Value, LambdaError> {
    let uuid = Uuid::new_v4().to_string(); //use this as a partition key for the DynamoDB table
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");

    //manage the credentials and get the region object.
    let config = aws_config::from_env().region(region_provider).load().await;

    //create dynamodb client object
    let client = Client::new(&config);

    let request = client
        .put_item()
        .table_name("customers")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item(
            "first_name",
            AttributeValue::S(String::from(event.first_name)),
        )
        .item(
            "last_name",
            AttributeValue::S(String::from(event.last_name)),
        )
        .item("ID", AttributeValue::N(event.id.to_string()));

    request.send().await?;
    Ok(json!({"message": "Record written!"}))
}
