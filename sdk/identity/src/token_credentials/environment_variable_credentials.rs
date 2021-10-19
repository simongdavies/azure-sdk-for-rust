use azure_core::errors::AzureError;
use azure_core::{TokenCredential, TokenResponse};
use chrono::{DateTime, NaiveDateTime, Utc};
use oauth2::AccessToken;
use serde::Deserialize;

use std::str;

const AZURE_MSI_TOKEN: &str = "AZURE_MSI_TOKEN";

///
/// Gets a token from an environment variable. Used in RPaaS to get MSI token for resource
///
///
pub struct EnvironmentVariableCredential;

#[async_trait::async_trait]
impl TokenCredential for EnvironmentVariableCredential {
    async fn get_token(&self, _resource: &str) -> Result<TokenResponse, AzureError> {
        let token = std::env::var(AZURE_MSI_TOKEN).map_err(|_| {
            AzureError::GenericErrorWithText(format!(
                "Missing environment variable {}",
                AZURE_MSI_TOKEN
            ))
        })?;

        let token_response = serde_json::from_str::<MsiTokenResponse>(&token)
            .map_err(|err| format!("Parsing Token Failed {}", err))?;

        let naive = NaiveDateTime::from_timestamp(
            token_response
                .expires_on
                .parse::<i64>()
                .map_err(|err| format!("Parsing expires_on from Token failed {}", err))?,
            0,
        );

        Ok(TokenResponse::new(
            token_response.access_token,
            DateTime::from_utc(naive, Utc),
        ))
    }
}

#[derive(Debug, Clone, Deserialize)]
struct MsiTokenResponse {
    pub access_token: AccessToken,
    pub expires_on: String,
    pub token_type: String,
    pub resource: String,
}
