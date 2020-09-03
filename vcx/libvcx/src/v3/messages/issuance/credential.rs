use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{Attachments, AttachmentId};
use v3::messages::ack::PleaseAck;
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;
use messages::payload::PayloadKinds;
use std::convert::TryInto;
use messages::issuance::credential::CredentialMessage;
use v3::messages::issuance::credential_offer::CredentialOffer;
use utils::libindy::types::CredentialOffer as IndyCredentialOffer;
use utils::libindy::types::Credential as IndyCredential;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Credential {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "credentials~attach")]
    pub credentials_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "~please_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub please_ack: Option<PleaseAck>,
}

impl Credential {
    pub fn create() -> Self {
        Credential::default()
    }

    pub fn set_comment(mut self, comment: Option<String>) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_credential(mut self, credential: String) -> VcxResult<Credential> {
        self.credentials_attach.add_base64_encoded_json_attachment(AttachmentId::Credential, ::serde_json::Value::String(credential))?;
        Ok(self)
    }

    pub fn ensure_match_offer(&self, offer: &CredentialOffer) -> VcxResult<()> {
        let indy_cred: IndyCredential = serde_json::from_str(&self.credentials_attach.content()?)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot parse Credential message from JSON string. Err: {:?}", err)))?;

        let indy_offer: IndyCredentialOffer = serde_json::from_str(&offer.offers_attach.content()?)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, format!("Cannot parse Credential Offer message from JSON string. Err: {:?}", err)))?;

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

        for attribute in offer.credential_preview.attributes.iter() {
            let received_cred_attribute = indy_cred.values.0.get(&attribute.name)
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                          format!("Invalid Credential: Cannot find \"{}\" attribute existing in the original Credential Offer.", attribute.name)))?;

            if !received_cred_attribute.raw.eq(&attribute.value) {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidCredential,
                                              format!("Invalid Credential: The value of \"{}\" attribute in Credential \
                                              does not match to the value \"{}\" of this attribute in the original Credential Offer.",
                                                      received_cred_attribute.raw, &attribute.value)));
            }
        }

        Ok(())
    }
}

please_ack!(Credential);
threadlike!(Credential);
a2a_message!(Credential);

impl TryInto<Credential> for CredentialMessage {
    type Error = VcxError;

    fn try_into(self) -> Result<Credential, Self::Error> {
        Credential::create()
            .set_thread_id(&self.claim_offer_id)
            .set_credential(self.libindy_cred)
    }
}

impl TryInto<CredentialMessage> for Credential {
    type Error = VcxError;

    fn try_into(self) -> Result<CredentialMessage, Self::Error> {
        let indy_credential_json = self.credentials_attach.content()?;

        let indy_credential: ::serde_json::Value = ::serde_json::from_str(&indy_credential_json)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot deserialize Indy Credential: {:?}", err)))?;

        Ok(CredentialMessage {
            msg_type: PayloadKinds::Cred.name().to_string(),
            libindy_cred: self.credentials_attach.content()?,
            claim_offer_id: self.thread.thid.clone().unwrap_or_default(),
            cred_revoc_id: None,
            revoc_reg_delta_json: None,
            version: String::from("0.1"),
            from_did: String::new(),
            cred_def_id: indy_credential["cred_def_id"].as_str().map(String::from).unwrap_or_default(),
            rev_reg_def_json: String::new(),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::issuance::credential_offer::tests::{thread, thread_id, _credential_offer};
    use v3::messages::issuance::CredentialValue;

    fn _attachment() -> ::serde_json::Value {
        json!({
            "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
            "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG1",
            "values":{"attribute":{"raw":"value","encoded":"1139481716457488690172217916278103335"}}
        })
    }

    fn _comment() -> Option<String> {
        Some(String::from("comment"))
    }

    pub fn _credential() -> Credential {
        let mut attachment = Attachments::new();
        attachment.add_base64_encoded_json_attachment(AttachmentId::Credential, _attachment()).unwrap();

        Credential {
            id: MessageId::id(),
            comment: _comment(),
            thread: thread(),
            credentials_attach: attachment,
            please_ack: None,
        }
    }

    #[test]
    fn test_credential_build_works() {
        let credential: Credential = Credential::create()
            .set_comment(_comment())
            .set_thread_id(&thread_id())
            .set_credential(_attachment().to_string()).unwrap();

        assert_eq!(_credential(), credential);
    }

    #[test]
    fn test_credential_match_offer() {
        // credential match offer
        _credential().ensure_match_offer(&_credential_offer()).unwrap();

        // credential does not match offer - attribute not found in credential
        let mut offer = _credential_offer();
        offer.credential_preview.attributes.push(CredentialValue{
            name: "other_attribute".to_string(),
            value: "value".to_string(),
            _type: None
        });
        _credential().ensure_match_offer(&offer).unwrap_err();

        // credential does not match offer - different value in credential
        let mut offer = _credential_offer();
        offer.credential_preview.attributes[0].value = "other_value".to_string();
        _credential().ensure_match_offer(&offer).unwrap_err();
    }
}