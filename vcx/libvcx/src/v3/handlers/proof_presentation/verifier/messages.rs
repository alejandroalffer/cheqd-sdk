use v3::messages::proof_presentation::presentation_proposal::PresentationProposal;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::error::ProblemReport;
use v3::messages::a2a::A2AMessage;
use v3::messages::proof_presentation::presentation_request::PresentationRequestData;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum VerifierMessages {
    PreparePresentationRequest(),
    SetConnection(u32),
    SendPresentationRequest(u32),
    PresentationReceived(Presentation),
    PresentationProposalReceived(PresentationProposal),
    PresentationRejectReceived(ProblemReport),
    RequestPresentation(u32, PresentationRequestData),
    Unknown
}

impl From<A2AMessage> for VerifierMessages {
    fn from(msg: A2AMessage) -> Self {
        match msg {
            A2AMessage::Presentation(presentation) => {
                VerifierMessages::PresentationReceived(presentation)
            }
            A2AMessage::PresentationProposal(presentation_proposal) => {
                VerifierMessages::PresentationProposalReceived(presentation_proposal)
            }
            A2AMessage::CommonProblemReport(report) |
            A2AMessage::PresentationReject(report)=> {
                VerifierMessages::PresentationRejectReceived(report)
            }
            _ => {
                VerifierMessages::Unknown
            }
        }
    }
}