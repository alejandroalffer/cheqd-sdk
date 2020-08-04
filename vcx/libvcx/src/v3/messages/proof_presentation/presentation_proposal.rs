use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::a2a::message_type::MessageType;
use v3::messages::a2a::message_family::MessageFamilies;
use v3::messages::mime_type::MimeType;
use messages::thread::Thread;
use messages::proofs::proof_request::{AttrInfo, Restrictions, PredicateInfo};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PresentationProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub presentation_proposal: PresentationPreview,
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
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
    pub cred_def_id: Option<String>,
    #[serde(rename = "mime-type")]
    pub mime_type: Option<MimeType>,
    pub value: Option<String>,
    pub filter: Option<Vec<::serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub cred_def_id: Option<String>,
    pub predicate: String,
    pub threshold: i64,
    pub filter: Option<Vec<::serde_json::Value>>,
}

fn default_presentation_preview_type() -> MessageType {
    MessageType::build(MessageFamilies::CredentialIssuance, "presentation-preview")
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
            attributes: vec![Attribute {
                name: String::from("name"),
                cred_def_id: None,
                mime_type: None,
                value: None,
                filter: None
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
    }
}
