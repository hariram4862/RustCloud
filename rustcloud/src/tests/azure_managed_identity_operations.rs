use crate::azure::azure_apis::security::azure_managed_identity::AzureManagedIdentityClient;

#[tokio::test]
async fn test_create_identity() {
    let client: AzureManagedIdentityClient = AzureManagedIdentityClient::new();

    let result = client
        .create_identity("test-rg", "rustcloud-identity", "eastasia")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create identity failed: {:?}", result);
}

#[tokio::test]
async fn test_list_identities_rg() {
    let client = AzureManagedIdentityClient::new();

    let result = client.list_identities_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List identities RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_identities_subscription() {
    let client = AzureManagedIdentityClient::new();

    let result = client.list_identities_subscription().await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List identities subscription failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_get_identity() {
    let client = AzureManagedIdentityClient::new();

    let result = client.get_identity("test-rg", "rustcloud-identity").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get identity failed: {:?}", result);
}

#[tokio::test]
async fn test_create_federated_credential() {
    let client = AzureManagedIdentityClient::new();

    let result = client
        .create_federated_credential(
            "test-rg",
            "rustcloud-identity",
            "rustcloud-fedcred",
            "https://token.actions.githubusercontent.com",
            "repo:org/repo:ref:refs/heads/main",
            "api://AzureADTokenExchange",
        )
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Create federated credential failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_list_federated_credentials() {
    let client = AzureManagedIdentityClient::new();

    let result = client
        .list_federated_credentials("test-rg", "rustcloud-identity")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List federated credentials failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_delete_federated_credential() {
    let client = AzureManagedIdentityClient::new();

    let result = client
        .delete_federated_credential("test-rg", "rustcloud-identity", "rustcloud-fedcred")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Delete federated credential failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_delete_identity() {
    let client = AzureManagedIdentityClient::new();

    let result = client
        .delete_identity("test-rg", "rustcloud-identity")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete identity failed: {:?}", result);
}
