/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateGpgKeyOption : CreateGPGKeyOption options create user GPG key



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateGpgKeyOption {
    /// An armored GPG key to add
    #[serde(rename = "armored_public_key")]
    pub armored_public_key: String,
    #[serde(rename = "armored_signature", skip_serializing_if = "Option::is_none")]
    pub armored_signature: Option<String>,
}

impl CreateGpgKeyOption {
    /// CreateGPGKeyOption options create user GPG key
    pub fn new(armored_public_key: String) -> CreateGpgKeyOption {
        CreateGpgKeyOption {
            armored_public_key,
            armored_signature: None,
        }
    }
}

