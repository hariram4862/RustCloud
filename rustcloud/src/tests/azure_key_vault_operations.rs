use crate::azure::azure_apis::security::azure_key_vault::AzureKeyVaultClient;

#[tokio::test]
async fn test_create_key_vault() {
    let client = AzureKeyVaultClient::new();

    let result = client
        .create_key_vault(
            "test-rg",
            "rustcloudkv123",
            "eastasia",
            "d4963ce2-af94-4122-95a9-644e8b01624d",
        )
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create Key Vault failed: {:?}", result);
}

#[tokio::test]
async fn test_list_key_vaults_rg() {
    let client = AzureKeyVaultClient::new();

    let result = client.list_key_vaults_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List Key Vaults RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_key_vaults_subscription() {
    let client = AzureKeyVaultClient::new();

    let result = client.list_key_vaults_subscription().await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List Key Vaults subscription failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_get_key_vault() {
    let client = AzureKeyVaultClient::new();

    let result = client.get_key_vault("test-rg", "rustcloudkv123").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get Key Vault failed: {:?}", result);
}

#[tokio::test]
async fn test_set_secret() {
    let client = AzureKeyVaultClient::new();

    let result = client
        .set_secret("rustcloudkv123", "demo-secret", "hello-rustcloud")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Set Secret failed: {:?}", result);
}

#[tokio::test]
async fn test_get_secret() {
    let client = AzureKeyVaultClient::new();

    let result = client.get_secret("rustcloudkv123", "demo-secret").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get Secret failed: {:?}", result);
}

#[tokio::test]
async fn test_list_secrets() {
    let client = AzureKeyVaultClient::new();

    let result = client.list_secrets("rustcloudkv123").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List Secrets failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_secret() {
    let client = AzureKeyVaultClient::new();

    let result = client.delete_secret("rustcloudkv123", "demo-secret").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete Secret failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_key_vault() {
    let client = AzureKeyVaultClient::new();

    let result = client.delete_key_vault("test-rg", "rustcloudkv123").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete Key Vault failed: {:?}", result);
}
