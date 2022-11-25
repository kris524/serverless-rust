
// get the curerntly configured aws chain
use aws_config::meta::region::RegionProviderChain;

// dynamodb part of the sdk
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;

//the lambda runtime that will help us tell lambda how to call our handler function
use lambda_runtime::{handler_fn, Context, Error as LambdaError};

// serde lib for deceralisation
use serde::Deserialize;

use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<(),LambdaError> {
   let func = handler_fn(handler);
   lambda_runtime::run(func).await?; //block until the results come back
        // the ? will get us the caller of the main function 
    Ok(())
}

//set up the custom event object 
