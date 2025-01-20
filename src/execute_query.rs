use actix_web::{web, HttpResponse, Error};
use snowflake_connector_rs::{SnowflakeClient, SnowflakeAuthMethod, SnowflakeClientConfig};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct QueryPayload {
    pub query: String,
}

pub async fn execute_query(payload: web::Json<QueryPayload>) -> Result<HttpResponse, Error> {
    dotenv().ok();
    // Read the credentials from environment variables
    let account = env::var("SNOWFLAKE_ACCOUNT").map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Missing SNOWFLAKE_ACCOUNT: {:?}", e))
    })?;
    let role = env::var("SNOWFLAKE_ROLE").ok();
    let warehouse = env::var("SNOWFLAKE_WAREHOUSE").ok();
    let database = env::var("SNOWFLAKE_DATABASE").ok();
    let schema = env::var("SNOWFLAKE_SCHEMA").ok();
    let user = env::var("SNOWFLAKE_USER").map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Missing SNOWFLAKE_USER: {:?}", e))
    })?;
    let password = env::var("SNOWFLAKE_PASSWORD").map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Missing SNOWFLAKE_PASSWORD: {:?}", e))
    })?;
   
    // Create the Snowflake client
    let client = SnowflakeClient::new(
        &user,
        SnowflakeAuthMethod::Password(password),
        SnowflakeClientConfig {
            account,
            role,
            warehouse,
            database,
            schema,
            timeout: Some(std::time::Duration::from_secs(0)),
        },
    ).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Client creation failed: {:?}", e))
    })?;
    let session = client.create_session().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Session creation failed: {:?}", e))
    })?;

    match session.query(payload.query.clone()).await {
        Ok(_) => {
            let response = ApiResponse {
                message: "Execution successful".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            let error_response = ApiResponse {
                message: format!("Query execution failed: {:?}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        },
    }
}

#[derive(serde::Serialize)]
struct ApiResponse {
    message: String,
}
