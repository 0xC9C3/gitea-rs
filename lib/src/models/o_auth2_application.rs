/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OAuth2Application {
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "confidential_client", skip_serializing_if = "Option::is_none")]
    pub confidential_client: Option<bool>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
}

impl OAuth2Application {
    pub fn new() -> OAuth2Application {
        OAuth2Application {
            client_id: None,
            client_secret: None,
            confidential_client: None,
            created: None,
            id: None,
            name: None,
            redirect_uris: None,
        }
    }
}


