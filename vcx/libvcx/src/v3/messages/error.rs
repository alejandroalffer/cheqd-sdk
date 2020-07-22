use v3::messages::a2a::{MessageId, A2AMessage};
use messages::thread::Thread;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProblemReport {
    #[serde(rename = "@id")]
    id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_retries: Option<WhoRetries>,
    #[serde(rename = "tracking-uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_uri: Option<String>,
    #[serde(rename = "escalation-uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_uri: Option<String>,
    #[serde(rename = "fix-hint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_hint: Option<FixHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<Impact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noticed_time: Option<String>,
    #[serde(rename = "where")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_items: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ProblemReport {
    pub fn create() -> Self {
        ProblemReport::default()
    }

    pub fn set_description(mut self, code: ProblemReportCodes) -> Self {
        self.description = Some(Description {
            en: code.en(),
            code: code.code(),
        });
        self
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

threadlike!(ProblemReport);
a2a_message!(ProblemReport, CommonProblemReport);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Description {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProblemReportCodes {
    Unimplemented,
    InvalidCredentialOffer,
    InvalidCredentialRequest,
    InvalidCredential,
    CredentialRejected,
    InvalidPresentationRequest,
    InvalidPresentation,
    PresentationRejected,
    Other(String)
}

impl ProblemReportCodes {
    fn code(&self) -> String {
        match self {
            ProblemReportCodes::Unimplemented => String::from("unimplemented"),
            ProblemReportCodes::InvalidCredentialOffer => String::from("invalid-credential-offer"),
            ProblemReportCodes::InvalidCredentialRequest => String::from("invalid-credential-request"),
            ProblemReportCodes::InvalidCredential => String::from("invalid-credential"),
            ProblemReportCodes::CredentialRejected => String::from("rejection"),
            ProblemReportCodes::InvalidPresentationRequest => String::from("invalid-request"),
            ProblemReportCodes::InvalidPresentation => String::from("invalid-presentation"),
            ProblemReportCodes::PresentationRejected => String::from("rejection"),
            ProblemReportCodes::Other(error) => error.to_string(),
        }
    }

    fn en(&self) -> Option<String> {
        match self {
            ProblemReportCodes::Unimplemented => Some(String::from("The protocol for received message is not implemented.")),
            ProblemReportCodes::InvalidCredentialOffer => Some(String::from("Couldn't create credential-request for received credential-offer.")),
            ProblemReportCodes::InvalidCredentialRequest => Some(String::from("Couldn't create credential for received credential-request.")),
            ProblemReportCodes::InvalidCredential => Some(String::from("Couldn't store received credential.")),
            ProblemReportCodes::CredentialRejected => Some(String::from("credential-offer was rejected.")),
            ProblemReportCodes::InvalidPresentationRequest => Some(String::from("Couldn't create presentation for received presentation-request.")),
            ProblemReportCodes::InvalidPresentation => Some(String::from("Couldn't verify presentation.")),
            ProblemReportCodes::PresentationRejected => Some(String::from("presentation-request was rejected.")),
            ProblemReportCodes::Other(_) => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WhoRetries {
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "you")]
    You,
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "none")]
    None,
}

impl Default for WhoRetries {
    fn default() -> WhoRetries {
        WhoRetries::None
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FixHint {
    en: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Impact {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "thread")]
    Thread,
    #[serde(rename = "connection")]
    Connection,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _code() -> String { String::from("test") }

    fn _comment() -> String {
        String::from("test comment")
    }

    pub fn _problem_report() -> ProblemReport {
        ProblemReport {
            id: MessageId::id(),
            thread: _thread(),
            description: Some(Description { en: None, code: _code() }),
            who_retries: None,
            tracking_uri: None,
            escalation_uri: None,
            fix_hint: None,
            impact: None,
            noticed_time: None,
            location: None,
            problem_items: None,
            comment: Some(_comment()),
        }
    }

    #[test]
    fn test_problem_report_build_works() {
        let report: ProblemReport = ProblemReport::default()
            .set_comment(_comment())
            .set_thread_id(&_thread_id())
            .set_description(ProblemReportCodes::Other(_code()));

        assert_eq!(_problem_report(), report);
    }
}