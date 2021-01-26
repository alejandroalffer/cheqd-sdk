use v3::messages::a2a::{MessageId, A2AMessage};
use proof::generate_nonce;
use error::VcxResult;
use messages::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Question {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub question_text: String,
    pub question_detail: Option<String>,
    pub nonce: String,
    pub signature_required: bool,
    pub valid_responses: Vec<QuestionResponse>,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuestionResponse {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Timing {
    pub expires_time: String,
}

impl Question {
    pub fn create() -> Question {
        Question::default()
    }

    pub fn set_question_text(mut self, question_text: String) -> Self {
        self.question_text = question_text;
        self
    }

    pub fn set_question_detail(mut self, question_detail: Option<String>) -> Self {
        self.question_detail = question_detail;
        self
    }

    pub fn set_nonce(mut self, nonce: Option<String>) -> VcxResult<Self> {
        self.nonce = match nonce {
            Some(nonce) => nonce,
            None => generate_nonce()?
        };
        Ok(self)
    }

    pub fn request_signature(mut self) -> Self {
        self.signature_required = true;
        self
    }

    pub fn set_valid_responses(mut self, valid_responses: Vec<QuestionResponse>) -> Self {
        self.valid_responses = valid_responses;
        self
    }

    pub fn set_expires_time(mut self, expires_time: String) -> Self {
        self.timing = Some(Timing {
            expires_time
        });
        self
    }
}

a2a_message!(Question);

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _question_text() -> String {
        String::from("Alice, are you on the phone with Bob from Faber Bank right now?")
    }

    fn _question_detail() -> String {
        String::from("This is optional fine-print giving context to the question and its various answers.")
    }

    fn _nonce() -> String {
        String::from("1000000")
    }

    fn _expires_time() -> String {
        String::from("2018-12-13T17:29:06+0000")
    }

    fn _valid_responses() -> Vec<QuestionResponse> {
        vec![
            QuestionResponse { text: "Yes, it's me".to_string() },
            QuestionResponse { text: "No, that's not me!".to_string() },
        ]
    }

    pub fn _question() -> Question {
        Question {
            id: MessageId::id(),
            question_text: _question_text(),
            question_detail: Some(_question_detail()),
            nonce: _nonce(),
            signature_required: true,
            valid_responses: _valid_responses(),
            timing: Some(Timing {
                expires_time: _expires_time()
            }),
            thread: None
        }
    }

    #[test]
    fn test_question_message_build_works() {
        let question: Question = Question::default()
            .set_question_text(_question_text())
            .set_question_detail(Some(_question_detail()))
            .request_signature()
            .set_valid_responses(_valid_responses())
            .set_expires_time(_expires_time())
            .set_nonce(Some(_nonce())).unwrap();
        assert_eq!(_question(), question);

        let expected = r#"{"@id":"testid","@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/questionanswer/1.0/question","nonce":"1000000","question_detail":"This is optional fine-print giving context to the question and its various answers.","question_text":"Alice, are you on the phone with Bob from Faber Bank right now?","signature_required":true,"valid_responses":[{"text":"Yes, it's me"},{"text":"No, that's not me!"}],"~timing":{"expires_time":"2018-12-13T17:29:06+0000"}}"#;
        assert_eq!(expected, json!(question.to_a2a_message()).to_string());
    }
}
