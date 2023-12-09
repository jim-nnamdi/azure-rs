use std::collections::BTreeMap;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserExtension {
  pub extension_playerid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserData {
  pub odata_context: String,
  pub value: Box<GetUserReturned>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserReturned {
  pub id: String,
  pub display_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AzRetrieveData {
  pub odatacontext:String,
  pub id:String,
  pub display_name:String,
  pub account_enabled: bool,
  pub given_name: String,
  pub mail:String,
  pub telephone_number:String,
  pub surname:String,
  pub created_date_time:String,
  pub success: bool,
  pub name: String,
  pub azauthed:String,
  pub identities:BTreeMap<&'static str, &'static str>,
  pub extensions: BTreeMap<&'static str, &'static str>,
  pub user_principal_name: String
}