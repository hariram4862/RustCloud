use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureKeyVaultClient {
    client: Client,
    subscription_id: String,
}

impl AzureKeyVaultClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_key_vault(
        &self,
        resource_group: &str,
        vault_name: &str,
        location: &str,
        tenant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2023-02-01",
            self.subscription_id,
            resource_group,
            vault_name
        );

        let body = serde_json::json!({
            "location": location,
            "properties": {
                "tenantId": tenant_id,
                "sku": {
                    "family": "A",
                    "name": "standard"
                },
                "accessPolicies": []
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

        println!("AZURE CREATE KEY VAULT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Key Vault failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_key_vaults_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults?api-version=2023-02-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST KEY VAULTS RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Key Vaults RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_key_vaults_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.KeyVault/vaults?api-version=2023-02-01",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST KEY VAULTS SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Key Vaults subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_key_vault(
        &self,
        resource_group: &str,
        vault_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2023-02-01",
            self.subscription_id,
            resource_group,
            vault_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET KEY VAULT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Key Vault failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_key_vault(
        &self,
        resource_group: &str,
        vault_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2023-02-01",
            self.subscription_id,
            resource_group,
            vault_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE KEY VAULT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Key Vault failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn set_secret(
        &self,
        vault_name: &str,
        secret_name: &str,
        secret_value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token_for_resource("https://vault.azure.net")?;

        let url = format!(
            "https://{}.vault.azure.net/secrets/{}?api-version=7.4",
            vault_name, secret_name
        );

        let body = serde_json::json!({
            "value": secret_value
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

        println!("AZURE SET SECRET");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Set Secret failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_secret(
        &self,
        vault_name: &str,
        secret_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token_for_resource("https://vault.azure.net")?;

        let url = format!(
            "https://{}.vault.azure.net/secrets/{}?api-version=7.4",
            vault_name, secret_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET SECRET");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Secret failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_secrets(&self, vault_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token_for_resource("https://vault.azure.net")?;

        let url = format!(
            "https://{}.vault.azure.net/secrets?api-version=7.4",
            vault_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST SECRETS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Secrets failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_secret(
        &self,
        vault_name: &str,
        secret_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token_for_resource("https://vault.azure.net")?;

        let url = format!(
            "https://{}.vault.azure.net/secrets/{}?api-version=7.4",
            vault_name, secret_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE SECRET");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Secret failed: {}", body).into());
        }

        Ok(())
    }
}
