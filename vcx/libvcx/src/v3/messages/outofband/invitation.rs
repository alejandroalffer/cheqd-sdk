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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileUrl")]
    pub profile_url: Option<String>,
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

    pub fn set_opt_profile_url(mut self, profile_url: Option<String>) -> Invitation {
        self.profile_url = profile_url;
        self
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
            profile_url: None
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
            profile_url: None
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

    #[test]
    fn test_outofband_invitation_from_json() {
        let invite = json!({
          "profileUrl": "https://freeiconshop.com/wp-content/uploads/edd/bank-flat.png",
          "public_did": "did:sov:LipMpSrURpZTiHb1PXX6jm",
          "request~attach": [{
            "mime-type": "application/didcomm-plain+json",
            "data": {"base64": "eyJjcmVkZW50aWFsX3ByZXZpZXciOnsiYXR0cmlidXRlcyI6W3sibmFtZSI6Im5hbWUiLCJ2YWx1ZSI6IkFsaWNlIFNtaXRoIn0seyJuYW1lIjoiZGVncmVlIiwidmFsdWUiOiJCYWNoZWxvcnMifV0sIkB0eXBlIjoiZGlkOnNvdjpCekNic05ZaE1yakhpcVpEVFVBU0hnO3NwZWMvaXNzdWUtY3JlZGVudGlhbC8xLjAvY3JlZGVudGlhbC1wcmV2aWV3In0sImNvbW1lbnQiOiJEZWdyZWUiLCJwcmljZSI6IjAiLCJvZmZlcnN+YXR0YWNoIjpbeyJkYXRhIjp7ImJhc2U2NCI6ImV5SnpZMmhsYldGZmFXUWlPaUpGU0VkYVVWWTJhbXBwWVdkTmVIRmxNVVZsTkVkbE9qSTZSR2x3Ykc5dFlTQTNZbUUzWkRKaU5Eb3dMakVpTENKamNtVmtYMlJsWmw5cFpDSTZJa1ZJUjFwUlZqWnFhbWxoWjAxNGNXVXhSV1UwUjJVNk16cERURG94TlRrNU56STZiR0YwWlhOMElpd2lhMlY1WDJOdmNuSmxZM1J1WlhOelgzQnliMjltSWpwN0ltTWlPaUkxTlRjMU5ETTJOalkzTWpVNU16azVORGt3TlRJNU1qYzNOVE15TnpFeU5UZzNNekF4TmpFNE16YzJOREk0TnpBMk5qTXpOekk0TkRBd01EVTNOVFF5T0RjMU16Z3dNVEk1TVRFMU5TSXNJbmg2WDJOaGNDSTZJams1TnpFNU1qSTFNVEUzTVRJNE16TTNOVGd4TnpRMU1qZ3dNVFF3TnpBd01UazBNVFUyTnpnNE9UazNOelF5TVRnek1ETTVNakV4TURjd01ESXdOREUyTnpVek5qSTBPVFk0T1RFeE16TTNNalU1TnpJd05URXdNelExTXpRNE1UQXlNakkxTmpnM056Y3lOemN3TURrNU1qVTVNRFE1TVRRNU9EWTVPVGN5TlRrNU5qZ3pNamN4T1RjM056RXpPRGszTmpVNE1ERTRNRFU1T0RVek56Z3lNVEkxTXpRM016TTNOVGd3TXpFd05URTVOalUyTWpNd05ESTNOVE13TURjeE9ETTBNVE01T0RVNE9USXlORGs1T1RneE9Ua3dOVGs1T1RBd056STBOVEUxTnpVMk9USTFOakl4TURBM05UWTFNemsxT1RRek9UTXlNVGc1TURRek16Y3lOek0zTlRFek5ETXhOelEwTWpnME56YzNNemcxTWpVd01EQTFOVFl3TURjeE5ERTVNamt4TlRBMk5EQXdOelk1TVRnM05qZzVOelF5TnpjNE9UZzVORFU0TkRNeU5URTFOamsxTlRNeU5qVTFOelk0TXpneE16WTNNalk1T1RnM09EVXhNRGMyTnprek9UTTNOakl5TmpjME1qY3dPVFUwT1RFME5EZzFPREUxTVRreE56SXlOVGMyTVRVMU5ETXpNek15T1RrMk9USTVNakE1TWpjM05UQTVNREUxTlRjek5UQTROVGM1TXpnek9EazJPRGN6TlRNNE16WXpNekU1TmpZek1qTTJORFUxTkRrd01UY3lPVFl6TkRFNU1UUXdNalF3TVRZNE5URTBOekUwTlRVeU9UWXhOVEl4TXpFNE5qQXlNekF4TURBMk5ETXdNelUzTkRNeU5USXlOalV3TXpneE9EZzBNVE13TkRneU9EYzJOREUwTWpJeU1UazJNRFl5TVRjeE1qTXdPVGcyTXpNek9Ua3lPREV6TnpNM05EWTFOVEEyTmpjM056TTNNRGs0TkRZNE5UQTRORFk1TURFNE9UVTJNRGs1TmpBNE9ERXdNVFkzT1RZME1qSXlOVEU0TkRFeE5qUTVNREUwTlRVNU5qQTNNVEEyTWpVNE5UYzVPRGt6T0RBNE9EY3dPRFEyTXpRM09UTXlNekl6TmpBd05qa3lPVGN4TkRNMk9UZzBOVE0wTmpNNE16WTJOU0lzSW5oeVgyTmhjQ0k2VzFzaVpHVm5jbVZsSWl3aU1UWTNOVEExTVRJM05qTXhOREF6TURNd01UUXhOREExTWpnNU9UWTFNVE14TmpreE1UTTFOalV3TlRNNE5EZzJPVGsyTWpnNE16WXpNamMzTXpNNE16TTVNekl6TWpjeE1UQTJNamszTWpZME5ERTVORGcxTmpVeE1ERTJOemN4TmpNeE5EYzFOelF5T1RrMk16Y3dOVGMyT1RVeU1qVTBPVFl4T0Rrd016Y3pNemcxTmpRNE5URTJNemt4TnpBME9UZ3lNemcwTkRnd05ETXhNelUwTURZM05qWXlOREkwTVRFd05ETTVORGd5TnpReU56Z3pNVFV4TVRVMU5qYzRNekEyTlRjeU16UXdNamsxTnpjME1EZzBNRGszTVRFeU5ETTBOVEF5TnpFek1ESTRNRGM1TVRJNE5EZzFPVEUxTVRneE16WTVPRFV5TVRrek1EUTFOVEUxTnpJME5qSXdOemczT0RjME5qa3dNVEE0T0RnMk16UTVPVEkyT0RJMU16VTFNakE1TlRrME1UTTVOalV6TkRVMk5qQTBOemd4T0RjNU5ESXhNRGM0TkRBM016SXlPVGd5TlRFeU1UY3lOakU1TnpneU56WTNOakE1TnpJME1ESTVPRE16TWpJeE16Z3dNRGcwT0RNMU5qTTNPRE0yTnpJeU56RXhOekE1TXpJNU56UTVOamM1TmpjNU56ZzBOekV6TWpVMk1EVTFNVEV5TnpjeE1UTTRNak0wTlRZeU16RXpNVFEwT0RreE5ERXpOamt3TlRNME1qRTNNVGMwTXpVNE9EVTVNVEF4TlRnMU9EUTBOekV5TnpReE9ESXhNVGM0TmpreU16QTBNamswTlRneU5UVTFOelUxTkRZMk9EZ3dNelUyT1RRd05qSXpOamM0TkRZd01qWXpNREkxTXpnMU56QTJNek0zTnpReE1EQTRPVGswTXpJNU5UazVNVFEzTlRJeE56azRPVGd5T0RNNE5ESTFNek14TkRNNE56STNNRFl6T0RRME5UZzNPRFk1TVRRME1qTXhORGd3T1Rjd056STJNRGN3TXpjM01ERTFOalU1TmpRMU16VTNORGd3TkRBNU5UWXdORGN5TURjeE9UZzRNems1TWpZeE1EUXhOREUwTURnd01UTXpNVFUxT0RBM05qZzJNREl3TmpReU5EYzVNVE15TmpZeE1qWTFNemcyTWpJeE5EVXhNek0wT0Rnd0lsMHNXeUp0WVhOMFpYSmZjMlZqY21WMElpd2lOVFl5TlRnME5EazBNalkwTURZeU5ESTNPVGcyTURBNU1USTBPVEF5TlRreE9ESXhNelF3TVRreU1EWTFORGd4T0RVMk1qZzFOVGs0T1RBME5EazRNemszTWpZMU9ERXlNVFExT0RFNE56SXlNRGt5T1RVNE16ZzVOelV4T0RVM01EZzFNelEyTVRVM01UZzNNalV4TVRFeE5EQTBNRGs1TkRReE1qQXdNalk0TXpjd01qRTJOREUwTnprd09UQTBNVFEzTnpReE56ZzVNakkwTkRZNE1EY3lNalEzT1RneE9EYzBNemc1TXpBd01UWTBNRGM0TnpnNE5UTTFNekk0TWpJeE5ERTJPVE16TXpZME5UVTJORGd4TlRjMk1UVTRNRFk1TURnMU9ESTJNell4TlRjME9Ea3dPVFU0TXpRM056WXlPVGs1TWpnd09UYzNOVEV5TmpRMk9UTXdOekV4T1RBMk1qQXhOamsyTnpBM09Ea3dNamMyT1RRMk1USXhOalkyTnpBd01ETXlPVGM1T0RJd09UVTVPRFUyTXpBeU5EYzJOek16T0RNeU5EZzVNVFl4T1RnNE9EZ3pPVE0zTmpnMk5qTTNOVGN4TnpreE1UUTBNREEzTXpFNU1EazVNalEzTURBMk9URTRORFV5T1RJM016WTNNRGczTkRjeU9UQTFOemczTkRjMU9EVXlNRFEwTkRFeU9URTVPREV6TkRnNE9UTXlPVFV6T0RjME9USTVPVFkyTnpJNU9ETTVNREF3T0RreU1qQTFORFUyT0RFNU1qVTNNREF5T0RNME9EY3lNRFV4T1RFek56SXhOVGN4TXpNMk5UUTRORFF4T1RNd01EUTBOakl5TnpjNE5UTTNNemswTURNME5EWTBNakU0TnpJeE1EWXlNemcwTVRNd05UazVNelEwTWpNM01UTXpNVGM1TURRNE5EQXlNVFl4TnpJNU5UTTJNVGd3T0RNeE9URTRNakV6TmpjME9UYzNOVE01TWprNE1UWTNNamcwTnpFd01EWTBNREUzTlRReE56YzFPVE0zT1RZeE1UazBNVE16TURFeU9USTFOVEV3TWpZek9EQTFNVE13TmpFek9EYzJNakkzT0RZME1UQXlNRE15TWpNek16RTJNVGc1T0RJek5ESTNNakE1TmpVek1EYzFOakEwTURFek1UZzJNall3T0RFM05ERXpOek0zTnpFeE56WTVJbDBzV3lKdVlXMWxJaXdpTkRrM05qSXlORGt5TWpFM09USTVOVE14T1RNeE5qazFNVFk0T0RVeE5UQXhOREEzTWpBek16VTRPRFF4TURNM09EUTFORGN3TWpBM01qWTRPRE0zTXpVNU1UQTNOVEUzTnpJd09UUTJOalU0TVRjek1EYzNNemd3TnpFeU56VXdNalkzTWprNU1qYzNNalkyTURneE16SXhNamN6TmpjeE5ETTVNVEkzTmpZME9UTXhOelF4TmpNMk9UazNORGN5Tmpjd016STBNamt4TlRjNU56WXdOVGt4T0Rnek9EQXhNRFUwTURjek5UQXpNekV5TnpZek5qazBOREF4T0RjeE1EYzBPREkxTmpRMU9Ua3hOekk1TmprM05qa3hNemswT0RjeE56UTROemMyTmpVeU9EazNORGMzTURZME1EZzJOREl3TlRJM09UUTVNelkwTnpFNE9URTVNalF3TXpnNU5EZzFNelEyTWpVNU5qQTJNREV3TXpNek56WXhOVFV5TXpjeE5URXpOak0zTkRNNE5UYzVNamd4TnpnNE1ESTNOemt4TlRJd05EWTNOell3TXpJMU9EWTJNalF6TVRZd05ERTNPREk1T0RjNU1UWXpOelV5TWprd09EUXpOREUwTWpBNU9UY3hPREEwTmpnek1ESTVOemMyTVRVM01UTXhNak13TVRJeU9EVXhOVGd3TURjM056WTVNek16TXpJNU16VTFNek0wTWpBeU5EWXpOamN6TVRNeE5EZzNOVFF5TURBeE56azNNemd5TnpVNU1UWTBOamt4TWpneU5UUXpOREE0TVRjME1UZ3dPRFExTlRJME9UazBOalU1TkRrMk1EYzRPRFEyTlRZek5ESTFPREV3TVRRNU5UZzVOakl5TVRVeE5EQXlOVGcwT0RNeE1ERTJNekk1TXpjNU1UWTVNREkyTVRFMU9UQXlPVFF5TWpNeU56QTVOekEzTlRZd09Ea3dNREUxTnpRNE1qQTNNemsxTWpZeE1qTTVNalU0TURRME5EazROekV4TmpBd01UTTVPRGt4T0RrNE9EUXdNalkwTnpjME56RXlPRFUyTXpJNU5qWTVNRGMxTnpNME16WTFORGs1TnpReE1qRXdNelkzTURJNE16ZzFNRE0xT1RBek9EUTVORGt6TURrMk56VXlOREF4T1RrNU1Ua3pOek0xTlRRMk56WXdNekk1TVRNd05EYzBNall3SWwxZGZTd2libTl1WTJVaU9pSTJPVGcxT1RRek16TXlNemd3T0RjeU5EUXdOVGM1TlRjaWZRPT0ifSwiQGlkIjoibGliaW5keS1jcmVkLW9mZmVyLTAiLCJtaW1lLXR5cGUiOiJhcHBsaWNhdGlvbi9qc29uIn1dLCJAdHlwZSI6ImRpZDpzb3Y6QnpDYnNOWWhNcmpIaXFaRFRVQVNIZztzcGVjL2lzc3VlLWNyZWRlbnRpYWwvMS4wL29mZmVyLWNyZWRlbnRpYWwiLCJAaWQiOiI5OTdkZTM2OS1jNDczLTRjZWMtOGEwZi1mYzdmMzJjMDkwNjUiLCJ+dGhyZWFkIjp7InRoaWQiOiJhYTc1ZDVhNi1hYWQwLTQ0OWItODRiZS1mYzc2YjEzMGIwM2UifX0="},
            "@id": "997de369-c473-4cec-8a0f-fc7f32c09065"
          }],
          "goal": "To issue a credential",
          "goal_code": "issue-vc",
          "service": [{
            "recipientKeys": ["GRj7miYEy2kD35UoQeg5ZvJ2M5hwBNkgQFQy2PJHrvs3"],
            "id": "VJbdEquEKv3mP4QWBYnbqg;indy",
            "routingKeys": [
              "GRj7miYEy2kD35UoQeg5ZvJ2M5hwBNkgQFQy2PJHrvs3",
              "DMinu4eYwd9mtpJ2BdhRVBzcopwegQizGqPYYZ9D37K9"
            ],
            "serviceEndpoint": "http://ad4d9dd03a34.ngrok.io:80/agency/msg",
            "type": "IndyAgent"
          }],
          "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/out-of-band/1.0/invitation",
          "handshake_protocols": ["did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/connections/1.0/"],
          "label": "Issuer",
          "@id": "71fa23b0-427e-4064-bf24-b375b1a2c64b"
        }).to_string();

        let _: Invitation = serde_json::from_str(&invite).unwrap();
    }
}