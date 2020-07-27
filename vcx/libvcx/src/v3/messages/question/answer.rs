use v3::messages::a2a::{MessageId, A2AMessage};
use chrono::prelude::*;
use messages::thread::Thread;
use v3::messages::a2a::message_type::MessageType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Answer {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub response: String,
    #[serde(rename = "~timing")]
    pub timing: Timing,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResponseSignature {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    pub signature: String,
    pub sig_data: String,
    pub signers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Timing {
    pub out_time: String,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing {
            out_time: format!("{:?}", Utc::now())
        }
    }
}

impl Answer {
    pub fn create() -> Answer {
        Answer::default()
    }

    pub fn set_response(mut self, response: String) -> Self {
        self.response = response;
        self
    }

    pub fn sign(self) -> Self {
        self
    }

    pub fn set_time(mut self, time: String) -> Self {
        self.timing = Timing {
            out_time: time
        };
        self
    }
}

threadlike!(Answer);
a2a_message!(Answer);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _response() -> String {
        String::from("Yes, it's me".to_string())
    }

    fn _time() -> String {
        String::from("2018-12-13T17:29:34+0000".to_string())
    }

    fn _answer() -> Answer {
        Answer {
            id: Default::default(),
            response: _response(),
            timing: Timing{
                out_time: _time()
            },
            thread: _thread(),
        }
    }

    #[test]
    fn test_answer_message_build_works() {
        let answer: Answer = Answer::default()
            .set_response(_response())
            .sign()
            .set_time(_time())
            .set_thread(_thread());

        assert_eq!(_answer(), answer);

        let expected = r#"{"@id":"testid","@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/questionanswer/1.0/answer","response":"Yes, it's me","~thread":{"received_orders":{},"sender_order":0,"thid":"test_id"},"~timing":{"out_time":"2018-12-13T17:29:34+0000"}}"#;
        assert_eq!(expected, json!(answer.to_a2a_message()).to_string());
    }
}