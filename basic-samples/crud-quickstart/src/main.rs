use azure_data_cosmos::{models::{ContainerProperties, PartitionKeyDefinition}, CosmosClient, PartitionKey};
use azure_identity::DefaultAzureCredential;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your actual Cosmos DB credentials
    let endpoint = "https://tvk-my-cosmos-account.documents.azure.com:443/";
    let credential = DefaultAzureCredential::new().unwrap();

    // Create a Cosmos client
    let client = CosmosClient::new(endpoint, credential, None)?;

    // Create a database (not support in RBAC)
    let database = "my-database".to_string();


    // Create a container
    let container = "my-container".to_string();
    let partition_key = "/id".to_string();
    let container_id = container;
    let properties = ContainerProperties {
        id: container_id.to_string(),
        partition_key: PartitionKeyDefinition::new(vec![partition_key]),
        ..Default::default()
    };
    client
    .database_client(database)
    .create_container(properties, None)
    .await?
    .deserialize_body()
    .await?
    .unwrap();

    Ok(())
}