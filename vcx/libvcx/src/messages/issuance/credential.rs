use std::collections::HashMap;
use messages::issuance::credential_offer::CredentialOffer;
use error::prelude::*;
use utils::libindy::types::Credential as IndyCredential;
use utils::libindy::types::CredentialOffer as IndyCredentialOffer;
use settings;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CredentialMessage {
    pub libindy_cred: String,
    pub rev_reg_def_json: String,
    pub cred_def_id: String,
    pub msg_type: String,
    pub claim_offer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_revoc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoc_reg_delta_json: Option<String>,
    pub version: String,
    pub from_did: String,
}

impl CredentialMessage {
    pub fn ensure_match_offer(&self, offer: &CredentialOffer) -> VcxResult<()> {
        if settings::indy_mocks_enabled() { return Ok(()) }

        let indy_cred: IndyCredential = serde_json::from_str(&self.libindy_cred)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                              format!("Cannot parse Credential message from JSON string. Err: {:?}", err)))?;

        let indy_offer: IndyCredentialOffer = serde_json::from_str(&offer.libindy_offer)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer,
                                              format!("Cannot parse Credential Offer message from JSON string. Err: {:?}", err)))?;

        if indy_cred.schema_id != indy_offer.schema_id {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                          format!("Invalid Credential: Credential `schema_id` \"{}\" does not match to `schema_id` \"{}\" in Credential Offer.",
                                                  indy_cred.schema_id,indy_offer.schema_id)));
        }

        if indy_cred.cred_def_id != indy_offer.cred_def_id {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                          format!("Invalid Credential: Credential `cred_def_id` \"{}\" does not match to `cred_def_id` \"{}\" in Credential Offer.",
                                                  indy_cred.schema_id,indy_offer.schema_id)));
        }

        for (key, value) in offer.credential_attrs.iter() {
            let received_cred_attribute = indy_cred.values.0.get(key)
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                          format!("Invalid Credential: Cannot find \"{}\" attribute existing in the original Credential Offer.", key))).unwrap();

            let value: &str = match value {
                // old style input such as {"address2":["101 Wilson Lane"]}
                serde_json::Value::Array(array_type) => {
                    array_type.get(0).and_then(serde_json::Value::as_str).unwrap_or_default()
                }

                // new style input such as {"address2":"101 Wilson Lane"}
                serde_json::Value::String(str_type) => str_type,
                // anything else is an error
                _ => {
                    return Err( VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer,
                                                   format!("Invalid Credential Offer format. Attribute values has unexpected value format: \"{}\"", value)));
                }
            };

            if !received_cred_attribute.raw.eq(value) {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                              format!("Invalid Credential: The value of \"{}\" attribute in Credential \
                                              does not match to the value \"{}\" of this attribute in the original Credential Offer.",
                                                      received_cred_attribute.raw, value)));
            }
        }

        Ok(())
    }
}
