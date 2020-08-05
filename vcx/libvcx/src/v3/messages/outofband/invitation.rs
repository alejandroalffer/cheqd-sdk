use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::attachment::{Attachments, AttachmentId};
use v3::messages::connection::did_doc::Service;
use error::prelude::*;

const SUPPORTED_HANDSHAKE_PROTOCOL: &str = "connections/1.0";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Invitation {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(default)]
    pub handshake_protocols: Vec<String>,
    #[serde(default)]
    #[serde(rename = "request~attach")]
    pub request_attach: Attachments,
    pub service: Vec<Service>,
}

impl Invitation {
    pub fn create() -> Invitation {
        Invitation::default()
    }

    pub fn set_id(mut self, id: String) -> Invitation {
        self.id = MessageId(id);
        self
    }

    pub fn set_label(mut self, label: String) -> Invitation {
        self.label = Some(label);
        self
    }

    pub fn set_goal_code(mut self, goal_code: String) -> Invitation {
        self.goal_code = Some(goal_code);
        self
    }

    pub fn set_opt_goal_code(mut self, goal_code: Option<String>) -> Invitation {
        self.goal_code = goal_code;
        self
    }

    pub fn set_goal(mut self, goal: String) -> Invitation {
        self.goal = Some(goal);
        self
    }

    pub fn set_opt_goal(mut self, goal: Option<String>) -> Invitation {
        self.goal = goal;
        self
    }

    pub fn set_handshake_protocol(mut self, handshake_protocol: String) -> Invitation {
        self.handshake_protocols.push(handshake_protocol);
        self
    }

    pub fn set_handshake(mut self, request_handshake: bool) -> Invitation {
        if request_handshake {
            // Out-of-Band RFC contains that format of handshake protocol for Connections protocol.
            // But it differs from format in Connection RFC where we use DID's
            self.handshake_protocols.push(String::from("https://didcomm.org/connections/1.0"));
//            self.handshake_protocols.push(MessageFamilies::Outofband.id());
        }
        self
    }

    pub fn set_service(mut self, service: Service) -> Invitation {
        self.service = vec![service];
        self
    }

    pub fn set_request_attach(mut self, attachment: String) -> VcxResult<Invitation> {
        self.request_attach.add_base64_encoded_json_attachment(AttachmentId::OutofbandRequest, ::serde_json::Value::String(attachment))?;
        Ok(self)
    }

    pub fn set_opt_request_attach(mut self, attachment: Option<String>) -> VcxResult<Invitation> {
        if let Some(attachment_) = attachment {
            self.request_attach.add_base64_encoded_json_attachment(AttachmentId::OutofbandRequest, ::serde_json::Value::String(attachment_))?;
        }
        Ok(self)
    }

    pub fn validate(&self) -> VcxResult<()> {
        if self.service.is_empty() {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidRedirectDetail,
                                          format!("Invalid Out-of-band Invitation: `service` is empty.`")));
        }

        if self.handshake_protocols.is_empty() && self.request_attach.0.is_empty() {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidRedirectDetail,
                                          format!("Invalid Out-of-band Invitation: `handshake_protocols` and `request~attach cannot be empty at the same time.`")));
        }

        if !self.handshake_protocols.is_empty() &&
            !self.handshake_protocols.iter().any(|protocol|  protocol.contains(SUPPORTED_HANDSHAKE_PROTOCOL)) {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidRedirectDetail,
                                          format!("Invalid Out-of-band Invitation: Could not find a supported `handshake_protocol`.\
                                          Requested: {:?}, Supported: {:?}`", self.handshake_protocols, SUPPORTED_HANDSHAKE_PROTOCOL)));
        }

        Ok(())
    }
}

a2a_message!(Invitation, OutOfBandInvitation);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::did_doc::tests::*;

    fn _attachment() -> ::serde_json::Value { json!({"request": {}}) }

    fn _attachment_json() -> String { _attachment().to_string() }

    fn _handshake_protocol() -> String { String::from("https://didcomm.org/connections/1.0") }

    fn _label() -> String { String::from("Faber College") }

    fn _goal_code() -> String { String::from("issue-vc") }

    fn _goal() -> String { String::from("To issue a Faber College Graduate credential") }

    pub fn _invitation() -> Invitation {
        let mut attachment = Attachments::new();
        attachment.add_base64_encoded_json_attachment(AttachmentId::OutofbandRequest, _attachment()).unwrap();

        Invitation {
            id: MessageId::id(),
            label: Some(_label()),
            goal_code: Some(_goal_code()),
            goal: Some(_goal()),
            handshake_protocols: vec![_handshake_protocol()],
            request_attach: attachment,
            service: vec![_service()],
        }
    }

    pub fn _invitation_no_handshake() -> Invitation {
        let mut attachment = Attachments::new();
        attachment.add_base64_encoded_json_attachment(AttachmentId::OutofbandRequest, _attachment()).unwrap();

        Invitation {
            id: MessageId::id(),
            label: Some(_label()),
            goal_code: Some(_goal_code()),
            goal: Some(_goal()),
            handshake_protocols: vec![],
            request_attach: attachment,
            service: vec![_service()],
        }
    }

    #[test]
    fn test_outofband_invitation_build_works() {
        let invitation: Invitation = Invitation::create()
            .set_label(_label())
            .set_goal(_goal())
            .set_goal_code(_goal_code())
            .set_handshake_protocol(_handshake_protocol())
            .set_service(_service())
            .set_request_attach(_attachment_json()).unwrap();

        assert_eq!(_invitation(), invitation);
    }

    #[test]
    fn test_outofband_invitation_validate_works() {
        _invitation().validate().unwrap();

        // only handshake_protocols
        Invitation::create()
            .set_label(_label())
            .set_service(_service())
            .set_handshake_protocol(_handshake_protocol())
            .validate().unwrap();

        // only request_attach
        Invitation::create()
            .set_label(_label())
            .set_service(_service())
            .set_request_attach(_attachment_json()).unwrap()
            .validate().unwrap();

        // missed handshake_protocols and  request_attach
        Invitation::create()
            .set_label(_label())
            .set_service(_service())
            .validate().unwrap_err();

        // missed service
        Invitation::create()
            .set_label(_label())
            .set_handshake_protocol(_handshake_protocol())
            .set_request_attach(_attachment_json()).unwrap()
            .validate().unwrap_err();
    }
}