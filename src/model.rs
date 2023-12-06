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
pub struct GetUserReturned {
  pub id: String,
  pub display_name: String,
}