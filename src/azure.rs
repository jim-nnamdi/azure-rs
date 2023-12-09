use std::collections::BTreeMap;

use crate::model::{GetUserData, UserExtension, AzRetrieveData};
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

pub async fn create_new_user(email:String, password:String, firstname:String, lastname:String, zipcode:String) -> Result<()>{
    // set the msal url for creating new user:preset in model
    let url = "https://graph.microsoft.com/v1.0/users";
    // BTreeMap for randomization of request data passed
    let mut data:BTreeMap<&str, Box<dyn std::any::Any>>  = BTreeMap::new();
    // Initiate the core-imp data to scaffold new user
    data.insert("firstname", Box::new(firstname));
    data.insert("surname", Box::new(lastname));
    data.insert("postalCode", Box::new(zipcode));
    // BTreeMap for sub-identity data msal model
    let mut identities:BTreeMap<&str, String> = BTreeMap::new();
    // Load identities information inside BTreeMap
    identities.insert("signInType", "emailAddress".to_owned());
    identities.insert("issuer", "host".to_owned());
    identities.insert("issuerAssignedId", email);
    // Dynamic Heap Allocation for identities data info
    data.insert("identities", Box::new(identities));
    // BTreeMap for password policies initilization
    let mut pass_policies:BTreeMap<&str, Box<dyn std::any::Any>> = BTreeMap::new();
    // Boxing data for password policies and mapping
    pass_policies.insert("password", Box::new(password));
    pass_policies.insert("forceChangePasswordNextSignIn", Box::new(false));
    data.insert("passwordProfile", Box::new(pass_policies));
    let azcreate = <Azureclient as AzureRequest>::make_post_request(url.to_owned(), data).await?;
    let regmap: AzRetrieveData = serde_json::from_str(&azcreate.as_str())?;
    Ok(())

}