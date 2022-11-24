
// get the curerntly configured aws chain
use aws_config::meta::region::RegionProviderChain;

// dynamodb part of the sdk
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;

//the lambda runtime that will help us tell lambda how to call our handler function
use lambda_runtime::{handler_fn, Context, Error as LambdaError};

// serde lib for deceralisation
use serde::Deserialize;





fn main() {
   
}
