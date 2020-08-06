use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::a2a::message_type::MessageType;
use v3::messages::a2a::message_family::MessageFamilies;
use v3::messages::mime_type::MimeType;
use messages::thread::Thread;
use messages::proofs::proof_request::{AttrInfo, Restrictions, PredicateInfo};
use utils::libindy::types::CredentialInfo;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PresentationProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub presentation_proposal: PresentationPreview,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationPreview {
    #[serde(rename = "@type")]
    #[serde(default = "default_presentation_preview_type")]
    pub _type: MessageType,
    pub attributes: Vec<Attribute>,
    pub predicates: Vec<Predicate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_def_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mime-type")]
    pub mime_type: Option<MimeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referent: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub cred_def_id: Option<String>,
    pub predicate: String,
    pub threshold: i64,
    pub referent: Option<String>,
}

fn default_presentation_preview_type() -> MessageType {
    MessageType::build(MessageFamilies::PresentProof, "presentation-preview")
}

impl PresentationProposal {
    pub fn create() -> Self {
        PresentationProposal::default()
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_presentation_preview(mut self, presentation_preview: PresentationPreview) -> PresentationProposal {
        self.presentation_proposal = presentation_preview;
        self
    }

    pub fn set_thread_id(mut self, id: &str) -> Self {
        self.thread = Some(Thread::new().set_thid(id.to_string()));
        self
    }

    pub fn to_proof_request_requested_attributes(&self) -> Vec<AttrInfo> {
        self.presentation_proposal.attributes
            .iter()
            .map(|attribute| AttrInfo {
                name: Some(attribute.name.clone()),
                names: None,
                restrictions: attribute.cred_def_id
                    .as_ref()
                    .map(|cred_def_id|
                        Restrictions::V2(json!({
                        "cred_def_id": cred_def_id
                    }))
                    ),
                non_revoked: None,
                self_attest_allowed: None,
            })
            .collect()
    }

    pub fn to_proof_request_requested_predicates(&self) -> Vec<PredicateInfo> {
        self.presentation_proposal.predicates
            .iter()
            .map(|predicate| PredicateInfo {
                name: predicate.name.clone(),
                p_type: predicate.predicate.clone(),
                p_value: predicate.threshold as i32,
                restrictions: predicate.cred_def_id
                    .as_ref()
                    .map(|cred_def_id|
                        Restrictions::V2(json!({
                        "cred_def_id": cred_def_id
                    }))
                    ),
                non_revoked: None,
            })
            .collect()
    }

    pub fn to_string(&self) -> String {
        json!(self.to_a2a_message()).to_string()
    }
}

impl Default for PresentationPreview {
    fn default() -> Self {
        PresentationPreview {
            _type: default_presentation_preview_type(),
            attributes: vec![],
            predicates: vec![]
        }
    }
}

impl PresentationPreview {
    pub fn create() -> Self {
        PresentationPreview::default()
    }

    pub fn for_credential(credential: &CredentialInfo) -> PresentationPreview {
        let attributes = credential.attrs
            .iter()
            .map(|(attribute, value)| Attribute {
                name: attribute.to_string(),
                cred_def_id: Some(credential.cred_def_id.to_string()),
                mime_type: None,
                value: Some(value.to_string()),
                referent: None,
            })
            .collect();

        PresentationPreview {
            attributes,
            ..PresentationPreview::default()
        }
    }
}

a2a_message!(PresentationProposal);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::proof_presentation::presentation_request::tests::{thread, thread_id};

    fn _attachment() -> ::serde_json::Value {
        json!({"presentation": {}})
    }

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn _presentation_preview() -> PresentationPreview {
        PresentationPreview {
            attributes: vec![Attribute{
                name: "account".to_string(),
                cred_def_id: Some("BzCbsNYhMrjHiqZDTUASHg:3:CL:1234:tag".to_string()),
                mime_type: None,
                value: Some("12345678".to_string()),
                referent: None
            }],
            predicates: vec![],
            ..Default::default()
        }
    }

    pub fn _presentation_proposal() -> PresentationProposal {
        PresentationProposal {
            id: MessageId::id(),
            comment: Some(_comment()),
            thread: Some(thread()),
            presentation_proposal: _presentation_preview(),
        }
    }

    #[test]
    fn test_presentation_proposal_build_works() {
        let presentation_proposal: PresentationProposal = PresentationProposal::default()
            .set_comment(_comment())
            .set_thread_id(&thread_id())
            .set_presentation_preview(_presentation_preview());

        assert_eq!(_presentation_proposal(), presentation_proposal);

        let expected = r#"{"@id":"testid","@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/propose-presentation","comment":"comment","presentation_proposal":{"@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/presentation-preview","attributes":[{"cred_def_id":"BzCbsNYhMrjHiqZDTUASHg:3:CL:1234:tag","name":"account","value":"12345678"}],"predicates":[]},"~thread":{"received_orders":{},"sender_order":0,"thid":"testid"}}"#;
        assert_eq!(expected.to_string(), presentation_proposal.to_string())
    }

    #[test]
    fn test_presentation_preview_for_credential_works() {
        // credential to use
        let credential = CredentialInfo {
            referent: "cred1".to_string(),
            attrs:  map!(
                "account".to_string() => "12345678".to_string(),
                "streetAddress".to_string() => "123 Main Street".to_string()
            ),
            schema_id: "2hoqvcwupRTUNkXn6ArYzs:2:schema_name:0.0.11".to_string(),
            cred_def_id: "BzCbsNYhMrjHiqZDTUASHg:3:CL:1234:tag".to_string(),
            rev_reg_id: None,
            cred_rev_id: None
        };

        // build presentation preview
        let presentation_preview = PresentationPreview::for_credential(&credential);

        assert_eq!(2, presentation_preview.attributes.len());

        let expected_attribute_1 = Attribute{
            name: "account".to_string(),
            cred_def_id: Some("BzCbsNYhMrjHiqZDTUASHg:3:CL:1234:tag".to_string()),
            mime_type: None,
            value: Some("12345678".to_string()),
            referent: None
        };

        let expected_attribute_2 = Attribute{
            name: "streetAddress".to_string(),
            cred_def_id: Some("BzCbsNYhMrjHiqZDTUASHg:3:CL:1234:tag".to_string()),
            mime_type: None,
            value: Some("123 Main Street".to_string()),
            referent: None
        };

        // check first attribute present
        presentation_preview.attributes
            .iter()
            .find(|attribute| expected_attribute_1.eq(attribute)).unwrap();

        // check second attribute present
        presentation_preview.attributes
            .iter()
            .find(|attribute| expected_attribute_2.eq(attribute)).unwrap();
    }
}
