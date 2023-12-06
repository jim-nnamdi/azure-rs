use crate::model::{GetUserData, UserExtension};
use crate::request::{AzureRequest, Azureclient};
use anyhow::{Ok, Result};

pub async fn get_user_extensions(session_key: &str) -> Result<UserExtension> {
    let url = format!("https://graph.microsoft.com/v1.0/users/{}?$select=extensions",session_key);
    let data = <Azureclient as AzureRequest>::fetch_get_request(url).await?;
    let structmap: UserExtension = serde_json::from_str(data.as_str())?;
    Ok(structmap)
}

pub async fn get_email(email: &str) -> Result<GetUserData> {
    let url = format!("https://graph.microsoft.com/v1.0/users?$select=displayName,id&$filter=identities/any(c:c/issuerAssignedId%20eq%20{}%20and%20c/issuer%20eq%20'cnerewardstest.onmicrosoft.com", email);
    let data = <Azureclient as AzureRequest>::fetch_get_request(url).await?;
    let structmap: GetUserData = serde_json::from_str(data.as_str())?;
    Ok(structmap)
}
