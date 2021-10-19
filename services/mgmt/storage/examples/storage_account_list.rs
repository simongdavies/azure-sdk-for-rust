/*
Lists the storage accounts, similar to:
az storage account list --query [].id

cargo run --example storage_account_list
*/
use azure_identity::token_credentials::EnvironmentVariableCredential;
use azure_mgmt_storage::operations::storage_accounts;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Requires Azure Subscription in AZURE_SUBSCRIPTION_ID and token in AZURE_MSI_TOKEN

#[tokio::main]
async fn main() -> Result<()> {
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> = std::sync::Arc::new(Box::new(reqwest::Client::new()));
    let token_credential = EnvironmentVariableCredential {};
    let subscription_id = env::var("AZURE_SUBSCRIPTION_ID").unwrap();

    // let subscription_id = &EnvironmentVariableCredential::get_subscription()?;
    let config = &azure_mgmt_storage::config(http_client, Box::new(token_credential)).build();

    let accounts = storage_accounts::list(config, &subscription_id).await?;
    println!("# of storage accounts {}", accounts.value.len());
    for account in &accounts.value {
        println!("{:?}", account.tracked_resource.resource.id);
    }
    Ok(())
}
