use settings::Actors;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, EnumIter)]
pub enum MessageFamilies {
    Routing,
    Connections,
    Notification,
    Signature,
    CredentialIssuance,
    ReportProblem,
    PresentProof,
    TrustPing,
    DiscoveryFeatures,
    Basicmessage,
    Outofband,
    Unknown(String)
}

impl MessageFamilies {
    pub const DID: &'static str = "did:sov:BzCbsNYhMrjHiqZDTUASHg";

    pub fn version(&self) -> &'static str {
        match self {
            MessageFamilies::Routing => "1.0",
            MessageFamilies::Connections => "1.0",
            MessageFamilies::Notification => "1.0",
            MessageFamilies::Signature => "1.0",
            MessageFamilies::CredentialIssuance => "1.0",
            MessageFamilies::ReportProblem => "1.0",
            MessageFamilies::PresentProof => "1.0",
            MessageFamilies::TrustPing => "1.0",
            MessageFamilies::DiscoveryFeatures => "1.0",
            MessageFamilies::Basicmessage => "1.0",
            MessageFamilies::Outofband => "1.0",
            MessageFamilies::Unknown(_) => "1.0"
        }
    }

    pub fn id(&self) -> String {
        format!("{};spec/{}/{}", Self::DID, self.to_string(), self.version().to_string())
    }

    pub fn actors(&self) -> Option<(Option<Actors>, Option<Actors>)> {
        match self {
            MessageFamilies::Routing => None,
            MessageFamilies::Connections => Some((Some(Actors::Inviter), Some(Actors::Invitee))),
            MessageFamilies::Notification => None,
            MessageFamilies::Signature => None,
            MessageFamilies::CredentialIssuance => Some((Some(Actors::Issuer), Some(Actors::Holder))),
            MessageFamilies::ReportProblem => None,
            MessageFamilies::PresentProof => Some((Some(Actors::Prover), Some(Actors::Verifier))),
            MessageFamilies::TrustPing => Some((Some(Actors::Sender), Some(Actors::Receiver))),
            MessageFamilies::DiscoveryFeatures => Some((Some(Actors::Sender), Some(Actors::Receiver))),
            MessageFamilies::Basicmessage => Some((Some(Actors::Sender), Some(Actors::Receiver))),
            MessageFamilies::Outofband => Some((None, Some(Actors::Receiver))),
            MessageFamilies::Unknown(_) => None
        }
    }
}

impl From<String> for MessageFamilies {
    fn from(family: String) -> Self {
        match family.as_str() {
            "routing" => MessageFamilies::Routing,
            "connections" => MessageFamilies::Connections,
            "signature" => MessageFamilies::Signature,
            "notification" => MessageFamilies::Notification,
            "issue-credential" => MessageFamilies::CredentialIssuance,
            "report-problem" => MessageFamilies::ReportProblem,
            "present-proof" => MessageFamilies::PresentProof,
            "trust_ping" => MessageFamilies::TrustPing,
            "discover-features" => MessageFamilies::DiscoveryFeatures,
            "basicmessage" => MessageFamilies::Basicmessage,
            "out-of-band" => MessageFamilies::Outofband,
            family @ _ => MessageFamilies::Unknown(family.to_string())
        }
    }
}

impl ::std::string::ToString for MessageFamilies {
    fn to_string(&self) -> String {
        match self {
            MessageFamilies::Routing => "routing".to_string(),
            MessageFamilies::Connections => "connections".to_string(),
            MessageFamilies::Notification => "notification".to_string(),
            MessageFamilies::Signature => "signature".to_string(),
            MessageFamilies::CredentialIssuance => "issue-credential".to_string(),
            MessageFamilies::ReportProblem => "report-problem".to_string(),
            MessageFamilies::PresentProof => "present-proof".to_string(),
            MessageFamilies::TrustPing => "trust_ping".to_string(),
            MessageFamilies::DiscoveryFeatures => "discover-features".to_string(),
            MessageFamilies::Basicmessage => "basicmessage".to_string(),
            MessageFamilies::Outofband => "out-of-band".to_string(),
            MessageFamilies::Unknown(family) => family.to_string()
        }
    }
}

impl Default for MessageFamilies {
    fn default() -> MessageFamilies {
        MessageFamilies::Unknown(String::new())
    }
}
