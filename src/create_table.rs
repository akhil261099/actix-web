use actix_web::{web, HttpResponse, Error};
use snowflake_connector_rs::{SnowflakeClient, SnowflakeAuthMethod, SnowflakeClientConfig};
use serde::Deserialize;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Deserialize)]
pub struct CreateTablePayload {
    pub create_query: String,
    pub insert_query: String,
}

pub async fn create_table_in_snowflake(payload: web::Json<CreateTablePayload>) -> Result<HttpResponse, Error> {
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

    // Execute the CREATE TABLE query 
    match session.query(payload.create_query.clone()).await {
        Ok(_) => (),
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to create table: {:?}", e),
            }));
        }
    };

    // Load and parse the CSV file
    let file_path = r"E:\rust\API\actix-web\src\snowflake_try.csv";
    let file = File::open(file_path).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to open CSV file: {:?}", e))
    })?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // Insert data from CSV
    for result in rdr.records() {
        let record = result.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("CSV read error: {:?}", e))
        })?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        let values_str = row.iter()
            .map(|field| format!("'{}'", field.replace("'", "''"))) 
            .collect::<Vec<String>>()
            .join(", ");

        // Construct and execute the INSERT SQL query
        let insert_sql = format!("{} VALUES ({})", payload.insert_query, values_str);
        match session.query(&*insert_sql).await {
            Ok(_) => (),
            Err(e) => {
                return Ok(HttpResponse::InternalServerError().json(ApiResponse {
                    message: format!("Insert query execution failed: {:?}", e),
                }));
            }
        }
    }

    // Return a success message
    Ok(HttpResponse::Ok().json(ApiResponse {
        message: "Table created and data inserted successfully".to_string(),
    }))
}

#[derive(serde::Serialize)]
struct ApiResponse {
    message: String,
}
