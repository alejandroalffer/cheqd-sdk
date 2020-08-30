use v3::messages::a2a::{A2AMessage, MessageId};
use messages::thread::Thread;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct HandshakeReuseAccepted {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl HandshakeReuseAccepted {
    pub fn create() -> HandshakeReuseAccepted {
        HandshakeReuseAccepted::default()
    }
}

threadlike!(HandshakeReuseAccepted);
a2a_message!(HandshakeReuseAccepted);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    pub fn _handshake_reuse_accepted() -> HandshakeReuseAccepted {
        HandshakeReuseAccepted {
            id: MessageId::id(),
            thread: _thread(),
        }
    }

    #[test]
    fn test_handshake_reuse_accepted_build_works() {
        let handshake_reuse_accepted = HandshakeReuseAccepted::default()
            .set_thread(_thread());

        assert_eq!(_handshake_reuse_accepted(), handshake_reuse_accepted);
    }
}