use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::ack::PleaseAck;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Invite {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub goal_code: String,
    #[serde(rename = "~please_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub please_ack: Option<PleaseAck>
}

impl Invite {
    pub fn create() -> Invite {
        Invite::default()
    }

    pub fn set_goal_code(mut self, goal_code: String) -> Invite {
        self.goal_code = goal_code;
        self
    }

    pub fn set_ack_on(mut self, ack_on: Option<Vec<String>>) -> Invite {
        if let Some(ack_on_) = ack_on {
            self.please_ack = Some(PleaseAck {
                on: Some(ack_on_)
            });
        }

        self
    }
}

a2a_message!(Invite, InviteForAction);
please_ack!(Invite);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InviteActionData {
    pub goal_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_on: Option<Vec<String>>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _goal_code() -> String {
        String::from("automotive.inspect.tire")
    }

    pub fn _invite() -> Invite {
        Invite {
            id: MessageId::id(),
            goal_code: _goal_code(),
            please_ack: None
        }
    }

    #[test]
    fn test_invite_build_works() {
        let invite: Invite = Invite::default()
            .set_goal_code(_goal_code());

        assert_eq!(_invite(), invite);

        let expected = r#"{"@id":"testid","@type":"https://didcomm.org/invite-action/0.9/invite","goal_code":"automotive.inspect.tire"}"#;
        assert_eq!(expected, json!(invite.to_a2a_message()).to_string());
    }
}