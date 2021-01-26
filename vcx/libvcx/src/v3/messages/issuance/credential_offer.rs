use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::issuance::CredentialPreviewData;
use v3::messages::attachment::{Attachments, AttachmentId};
use v3::messages::mime_type::MimeType;
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;
use messages::issuance::credential_offer::CredentialOffer as CredentialOfferV1;
use messages::payload::PayloadKinds;
use std::convert::TryInto;
use utils::libindy::anoncreds::ensure_credential_definition_contains_offered_attributes;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CredentialOffer {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub credential_preview: CredentialPreviewData,
    #[serde(rename = "offers~attach")]
    pub offers_attach: Attachments,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
}

impl CredentialOffer {
    pub fn create() -> Self {
        CredentialOffer::default()
    }

    pub fn set_id(mut self, id: String) -> Self {
        self.id = MessageId(id);
        self
    }

    pub fn set_comment(mut self, comment: Option<String>) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_offers_attach(mut self, credential_offer: &str) -> VcxResult<CredentialOffer> {
        self.offers_attach.add_base64_encoded_json_attachment(AttachmentId::CredentialOffer, ::serde_json::Value::String(credential_offer.to_string()))?;
        Ok(self)
    }

    pub fn set_credential_preview_data(mut self, credential_preview: CredentialPreviewData) -> VcxResult<CredentialOffer> {
        self.credential_preview = credential_preview;
        Ok(self)
    }

    pub fn add_credential_preview_data(mut self, name: &str, value: &str, mime_type: MimeType) -> VcxResult<CredentialOffer> {
        self.credential_preview = self.credential_preview.add_value(name, value, mime_type)?;
        Ok(self)
    }

    pub fn set_thread_id(mut self, id: &str) -> Self {
        self.thread = Some(Thread::new().set_thid(id.to_string()));
        self
    }

    pub fn ensure_match_credential_definition(&self, cred_def_json: &str) -> VcxResult<()> {
        let cred_offer_attributes = self.credential_preview.attributes.iter().map(|value| &value.name).collect();
        ensure_credential_definition_contains_offered_attributes(cred_def_json, cred_offer_attributes)
    }
}

a2a_message!(CredentialOffer);

impl TryInto<CredentialOffer> for CredentialOfferV1 {
    type Error = VcxError;

    fn try_into(self) -> Result<CredentialOffer, Self::Error> {
        let mut credential_preview = CredentialPreviewData::new();

        for (key, value) in self.credential_attrs {
            credential_preview = credential_preview.add_value(&key, &value.as_str().unwrap_or_default(), MimeType::Plain)?;
        }

        CredentialOffer::create()
            .set_id(self.thread_id.unwrap_or_default())
            .set_credential_preview_data(credential_preview)?
            .set_offers_attach(&self.libindy_offer)
    }
}

impl TryInto<CredentialOfferV1> for CredentialOffer {
    type Error = VcxError;

    fn try_into(self) -> Result<CredentialOfferV1, Self::Error> {
        let indy_cred_offer_json = self.offers_attach.content()?;
        let indy_cred_offer: ::serde_json::Value = ::serde_json::from_str(&indy_cred_offer_json)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, format!("Cannot deserialize Indy Offer: {:?}", err)))?;

        let mut credential_attrs: ::serde_json::Map<String, ::serde_json::Value> = ::serde_json::Map::new();

        for attr in self.credential_preview.attributes {
            credential_attrs.insert(attr.name.clone(), ::serde_json::Value::String(attr.value.clone()));
        }

        let thid = self.thread.and_then(|thread| thread.thid).unwrap_or(self.id.0.clone());

        Ok(CredentialOfferV1 {
            msg_type: PayloadKinds::CredOffer.name().to_string(),
            version: String::from("0.1"),
            to_did: String::new(),
            from_did: String::new(),
            credential_attrs,
            schema_seq_no: 0,
            claim_name: self.comment.unwrap_or("".to_string()),
            claim_id: String::new(),
            msg_ref_id: None,
            cred_def_id: indy_cred_offer["cred_def_id"].as_str().map(String::from).unwrap_or_default(),
            libindy_offer: indy_cred_offer_json,
            thread_id: Some(thid),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::constants::CRED_DEF_JSON;

    fn _attachment() -> ::serde_json::Value {
        json!({
            "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
            "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG1"
        })
    }

    fn _comment() -> Option<String> {
        Some(String::from("comment"))
    }

    pub fn _value() -> (&'static str, &'static str) {
        ("attribute", "value")
    }

    pub fn _preview_data() -> CredentialPreviewData {
        let (name, value) = _value();
        CredentialPreviewData::new()
            .add_value(name, value, MimeType::Plain).unwrap()
    }

    pub fn thread() -> Thread {
        Thread::new().set_thid(_credential_offer().id.0)
    }

    pub fn thread_id() -> String {
        thread().thid.unwrap()
    }

    pub fn _credential_offer() -> CredentialOffer {
        let mut attachment = Attachments::new();
        attachment.add_base64_encoded_json_attachment(AttachmentId::CredentialOffer, _attachment()).unwrap();

        CredentialOffer {
            id: MessageId::id(),
            comment: _comment(),
            credential_preview: _preview_data(),
            offers_attach: attachment,
            thread: None,
        }
    }

    #[test]
    fn test_credential_offer_build_works() {
        let credential_offer: CredentialOffer = CredentialOffer::create()
            .set_comment(_comment())
            .set_credential_preview_data(_preview_data()).unwrap()
            .set_offers_attach(&_attachment().to_string()).unwrap();

        assert_eq!(_credential_offer(), credential_offer);
    }

    #[test]
    fn test_credential_offer_match_cred_def_works() {
        // Credential Definition contains attributes: name, height, sex, age

        // Credential Offer contains less attributes than Credential Definition
        let credential_offer: CredentialOffer = CredentialOffer::create()
            .set_credential_preview_data(
                CredentialPreviewData::new()
                    .add_value("name", "Test", MimeType::Plain).unwrap()
            ).unwrap();

        credential_offer.ensure_match_credential_definition(CRED_DEF_JSON).unwrap_err();

        // Credential Offer contains same attributes as Credential Definition
        let credential_offer: CredentialOffer = CredentialOffer::create()
            .set_credential_preview_data(
                CredentialPreviewData::new()
                    .add_value("name", "Test", MimeType::Plain).unwrap()
                    .add_value("height", "Test", MimeType::Plain).unwrap()
                    .add_value("sex", "Test", MimeType::Plain).unwrap()
                    .add_value("age", "Test", MimeType::Plain).unwrap()
            ).unwrap();

        credential_offer.ensure_match_credential_definition(CRED_DEF_JSON).unwrap();

        // Credential Offer contains same attributes as Credential Definition but in different case
        let credential_offer: CredentialOffer = CredentialOffer::create()
            .set_credential_preview_data(
                CredentialPreviewData::new()
                    .add_value("NAME", "Test", MimeType::Plain).unwrap()
                    .add_value("Height", "Test", MimeType::Plain).unwrap()
                    .add_value("SEX", "Test", MimeType::Plain).unwrap()
                    .add_value("age", "Test", MimeType::Plain).unwrap()
            ).unwrap();

        credential_offer.ensure_match_credential_definition(CRED_DEF_JSON).unwrap();

        // Credential Offer contains more attributes than Credential Definition
        let credential_offer: CredentialOffer = CredentialOffer::create()
            .set_credential_preview_data(
                CredentialPreviewData::new()
                    .add_value("name", "Test", MimeType::Plain).unwrap()
                    .add_value("height", "Test", MimeType::Plain).unwrap()
                    .add_value("sex", "Test", MimeType::Plain).unwrap()
                    .add_value("age", "Test", MimeType::Plain).unwrap()
                    .add_value("additional", "Test", MimeType::Plain).unwrap()
            ).unwrap();

        credential_offer.ensure_match_credential_definition(CRED_DEF_JSON).unwrap_err();
    }
}