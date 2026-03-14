use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureManagedIdentityClient {
    client: Client,
    subscription_id: String,
}

impl AzureManagedIdentityClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_identity(
        &self,
        resource_group: &str,
        identity_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name
        );

        let body = serde_json::json!({
            "location": location
        });

        let res = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE MANAGED IDENTITY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Identity failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_identities_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities?api-version=2023-01-31",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST IDENTITIES RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Identities RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_identities_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities?api-version=2023-01-31",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST IDENTITIES SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Identities Subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_identity(
        &self,
        resource_group: &str,
        identity_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET MANAGED IDENTITY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Identity failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_identity(
        &self,
        resource_group: &str,
        identity_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE MANAGED IDENTITY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Identity failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn create_federated_credential(
        &self,
        resource_group: &str,
        identity_name: &str,
        credential_name: &str,
        issuer: &str,
        subject: &str,
        audience: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
"https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}/federatedIdentityCredentials/{}?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name,
            credential_name
        );

        let body = serde_json::json!({
            "properties": {
                "issuer": issuer,
                "subject": subject,
                "audiences": [audience]
            }
        });

        let res = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE FEDERATED CREDENTIAL");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Federated Credential failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_federated_credentials(
        &self,
        resource_group: &str,
        identity_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
"https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}/federatedIdentityCredentials?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST FEDERATED CREDENTIALS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Federated Credentials failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_federated_credential(
        &self,
        resource_group: &str,
        identity_name: &str,
        credential_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
"https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}/federatedIdentityCredentials/{}?api-version=2023-01-31",
            self.subscription_id,
            resource_group,
            identity_name,
            credential_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE FEDERATED CREDENTIAL");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Federated Credential failed: {}", body).into());
        }

        Ok(())
    }
}
