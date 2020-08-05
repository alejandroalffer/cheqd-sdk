use v3::messages::a2a::{A2AMessage, MessageId};
use messages::thread::Thread;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct HandshakeReuse {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl HandshakeReuse {
    pub fn create() -> HandshakeReuse {
        HandshakeReuse::default()
    }
}

threadlike!(HandshakeReuse);
a2a_message!(HandshakeReuse);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    pub fn _handshake_reuse() -> HandshakeReuse {
        HandshakeReuse {
            id: MessageId::id(),
            thread: _thread(),
        }
    }

    #[test]
    fn test_handshake_reuse_build_works() {
        let handshake_reuse = HandshakeReuse::default()
            .set_thread(_thread());

        assert_eq!(_handshake_reuse(), handshake_reuse);
    }
}