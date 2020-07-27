use v3::messages::a2a::{MessageId, A2AMessage};
use messages::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Answer {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "response.@sig")]
    pub signature: ResponseSignature,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResponseSignature {
    pub signature: String,
    pub sig_data: String,
    pub timestamp: String,
}

impl Answer {
    pub fn create() -> Answer {
        Answer::default()
    }

    pub fn sign() -> Self {
        unimplemented!()
    }

    pub fn set_signature(mut self, signature: ResponseSignature) -> Self {
        self.signature = signature;
        self
    }
    
    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::CommitedAnswer(self.clone())
    }
}

threadlike!(Answer);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _answer_text() -> String {
        String::from("Yes, it's me".to_string())
    }

    fn _time() -> String {
        String::from("2018-12-13T17:29:34+0000".to_string())
    }

    fn _answer() -> Answer {
        Answer {
            id: Default::default(),
            thread: _thread(),
            signature: Default::default()
        }
    }

    #[test]
    fn test_answer_message_build_works() {
        let answer: Answer = Answer::default()
            .set_thread(_thread());

        assert_eq!(_answer(), answer);

        let expected = r#"{"@id":"testid","@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/committedanswer/1.0/answer-given","response.@sig":{"sig_data":"","signature":"","timestamp":""},"~thread":{"received_orders":{},"sender_order":0,"thid":"test_id"}}"#;
        assert_eq!(expected, json!(answer.to_a2a_message()).to_string());
    }
}
