
pub static EXT_URL: &str = "https://graph.microsoft.com/v1.0/users/{}?$select=extensions";
pub static EMAIL_URL: &str = "https://graph.microsoft.com/v1.0/users?$select=displayName,id&$filter=identities/any(c:c/issuerAssignedId%20eq%20{}%20and%20c/issuer%20eq%20'cnerewardstest.onmicrosoft.com";