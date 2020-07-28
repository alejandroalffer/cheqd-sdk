use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::SignedResponse;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::trust_ping::ping::Ping;
use v3::messages::trust_ping::ping_response::PingResponse;
use v3::messages::ack::Ack;
use v3::messages::discovery::query::Query;
use v3::messages::discovery::disclose::Disclose;
use v3::messages::a2a::A2AMessage;
use v3::messages::outofband::invitation::Invitation as OutofbandInvitation;
use v3::messages::outofband::handshake_reuse::HandshakeReuse;
use v3::messages::outofband::handshake_reuse_accepted::HandshakeReuseAccepted;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeMessages {
    Connect(),
    InvitationReceived(Invitation),
    ExchangeRequestReceived(Request),
    ExchangeResponseReceived(SignedResponse),
    AckReceived(Ack),
    ProblemReportReceived(ProblemReport),
    SendPing(Option<String>),
    PingReceived(Ping),
    PingResponseReceived(PingResponse),
    DiscoverFeatures((Option<String>, Option<String>)),
    QueryReceived(Query),
    OutofbandInvitationReceived(OutofbandInvitation),
    SendHandshakeReuse(OutofbandInvitation),
    HandshakeReuseReceived(HandshakeReuse),
    HandshakeReuseAcceptedReceived(HandshakeReuseAccepted),
    DiscloseReceived(Disclose),
    Unknown
}

impl From<A2AMessage> for DidExchangeMessages {
    fn from(msg: A2AMessage) -> Self {
        match msg {
            A2AMessage::ConnectionInvitation(invite) => {
                DidExchangeMessages::InvitationReceived(invite)
            }
            A2AMessage::ConnectionRequest(request) => {
                DidExchangeMessages::ExchangeRequestReceived(request)
            }
            A2AMessage::ConnectionResponse(request) => {
                DidExchangeMessages::ExchangeResponseReceived(request)
            }
            A2AMessage::Ping(ping) => {
                DidExchangeMessages::PingReceived(ping)
            }
            A2AMessage::PingResponse(ping_response) => {
                DidExchangeMessages::PingResponseReceived(ping_response)
            }
            A2AMessage::Ack(ack) => {
                DidExchangeMessages::AckReceived(ack)
            }
            A2AMessage::Query(query) => {
                DidExchangeMessages::QueryReceived(query)
            }
            A2AMessage::Disclose(disclose) => {
                DidExchangeMessages::DiscloseReceived(disclose)
            }
            A2AMessage::HandshakeReuse(handshake_reuse) => {
                DidExchangeMessages::HandshakeReuseReceived(handshake_reuse)
            }
            A2AMessage::HandshakeReuseAccepted(handshake_reuse_accepted) => {
                DidExchangeMessages::HandshakeReuseAcceptedReceived(handshake_reuse_accepted)
            }
            A2AMessage::ConnectionProblemReport(report) => {
                DidExchangeMessages::ProblemReportReceived(report)
            }
            _ => {
                DidExchangeMessages::Unknown
            }
        }
    }
}