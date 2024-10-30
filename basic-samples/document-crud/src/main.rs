use azure_core::StatusCode;
use azure_data_cosmos::CosmosClient;
use azure_identity::DefaultAzureCredential;
use tokio;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your actual Cosmos DB credentials

    // Load cosmos db environment variables
    match env::var("COSMOSDB_ENDPOINT") {
        Ok(value) => println!("COSMOSDB_ENDPOINT is: {}", value),
        Err(e) => println!("Couldn't read COSMOSDB_ENDPOINT ({})", e),
    }

    let endpoint = env::var("COSMOSDB_ENDPOINT").unwrap();
    let credential = DefaultAzureCredential::new().unwrap();

    // Create a Cosmos client
    let client = CosmosClient::new(endpoint, credential, None)?;

    // Set database (create database not support in RBAC)
    let database = "database-name".to_string();

    // Set container (ensure this container already exists ith partition key of "/pk")
    let container = "container-name".to_string();

    let db_client = client.database_client(database);
    let container_client = db_client.container_client(container);

    let item = serde_json::json!({
        "id": "my-item-id-value",
        "pk": "my-item-partition-key-value",
        "name": "my-item-name",
    });

    // create item
    match container_client.create_item("my-item-partition-key-value", item, None).await {
        Ok(create_response) => {
            let created_item = create_response.deserialize_body().await?;
            println!("Created item: {:#?}", created_item);
        }
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            return Err(format!("Document with ID 'my-item-id-value' already exists.").into());
        }
        Err(e) => return Err(e.into()),
    }

    // read item
    let read_response = container_client
    .read_item("my-item-partition-key-value", "my-item-id-value", None)
    .await;
    match read_response {
        Err(e) if e.http_status() == Some(StatusCode::NotFound) => println!("Item not found!"),
        Ok(r) => {
            let item: serde_json::Value = r.deserialize_body().await?.unwrap();
            println!("Found item:");
            println!("{:#?}", item);
        }
        Err(e) => return Err(e.into()),
    };

    // upsert item
    let upsert_item = serde_json::json!({
        "id": "my-item-id-value",
        "pk": "my-item-partition-key-value",
        "name": "my-item-name-updated",
    });
    let upsert_response = container_client
    .upsert_item("my-item-partition-key-value", upsert_item, None)
    .await?
    .deserialize_body()
    .await?
    .unwrap();
    println!("Upsert item:");
    println!("{:#?}", upsert_response);

    // delete item
    let delete_response = container_client
    .delete_item("my-item-partition-key-value", "my-item-id-value", None)
    .await;
    match delete_response {
        Err(e) if e.http_status() == Some(StatusCode::NotFound) => println!("Item not found!"),
        Ok(_) => println!("Item deleted"),
        Err(e) => return Err(e.into()),
    };
    
    Ok(())
}