use actix_web::{web, HttpResponse, Error};
use snowflake_connector_rs::{SnowflakeClient, SnowflakeAuthMethod, SnowflakeClientConfig};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryPayload {
    pub query: String,
}

pub async fn execute_query(payload: web::Json<QueryPayload>) -> Result<HttpResponse, Error> {
    let client = SnowflakeClient::new(
        "akhil969",
        SnowflakeAuthMethod::Password("zaq1ZAQ1@1".to_string()),
        SnowflakeClientConfig {
            account: "GMGNHXO-WM55675".to_string(), 
            role: Some("ACCOUNTADMIN".to_string()),
            warehouse: Some("COMPUTE_WH".to_string()),
            database: Some("TRAININGDB".to_string()),
            schema: Some("SALES".to_string()),
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
