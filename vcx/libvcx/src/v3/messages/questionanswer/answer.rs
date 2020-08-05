use v3::messages::a2a::{MessageId, A2AMessage};
use chrono::prelude::*;
use messages::thread::Thread;
use v3::messages::a2a::message_type::MessageType;
use v3::messages::a2a::message_family::MessageFamilies;
use v3::messages::questionanswer::question::Question;
use utils::libindy::crypto;
use error::prelude::*;
use sha2::{Sha512, Digest};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Answer {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub response: String,
    #[serde(rename = "~timing")]
    pub timing: Timing,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response~sig")]
    pub response_sig: Option<ResponseSignature>,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSignature {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    pub signature: String,
    pub sig_data: String,
    pub signers: Vec<String>,
}

impl Default for ResponseSignature {
    fn default() -> ResponseSignature {
        ResponseSignature {
            msg_type: MessageType::build(MessageFamilies::QuestionAnswer, "ed25519Sha512_single"),
            signature: String::new(),
            sig_data: String::new(),
            signers: Vec::new(),
        }
    }
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

    pub fn sign(mut self, question: &Question, key: &str) -> VcxResult<Self> {
        trace!("Answer::sign >>> question: {:?}", secret!(question));

        let mut sig_data = question.question_text.as_bytes().to_vec();
        sig_data.extend(self.response.as_bytes());
        sig_data.extend(question.nonce.as_bytes());

        let mut hasher = Sha512::new();
        hasher.update(&sig_data);
        let sig_data = hasher.finalize();

        let signature = crypto::sign(key, &sig_data)?;

        let sig_data = base64::encode_config(&sig_data, base64::URL_SAFE);

        let signature = base64::encode_config(&signature, base64::URL_SAFE);

        self.response_sig = Some(ResponseSignature {
            signature,
            sig_data,
            signers: vec![key.to_string()],
            ..Default::default()
        });

        trace!("Answer::sign <<<");
        Ok(self)
    }

    pub fn verify(&self, key: &str) -> VcxResult<()>{
        trace!("Answer::verify >>> self: {:?}", secret!(self));

        if let Some(ref response_sig) = self.response_sig.as_ref() {
            let signature = base64::decode_config(&response_sig.signature.as_bytes(), base64::URL_SAFE)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot decode AnswerSignature: {:?}", err)))?;

            let sig_data = base64::decode_config(&response_sig.sig_data.as_bytes(), base64::URL_SAFE)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot decode AnswerSignature: {:?}", err)))?;

            if !crypto::verify(&key, &sig_data, &signature)? {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "Answer signature is invalid for pairwise key"));
            }
        }

        trace!("Answer::verify <<<");
        Ok(())
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
    use utils::libindy::tests::test_setup;
    use v3::messages::questionanswer::question::tests::_question;

    fn _response() -> String {
        String::from("Yes, it's me".to_string())
    }

    fn _time() -> String {
        String::from("2018-12-13T17:29:34+0000".to_string())
    }

    pub fn _answer() -> Answer {
        Answer {
            id: Default::default(),
            response: _response(),
            timing: Timing{
                out_time: _time()
            },
            response_sig: None,
            thread: _thread(),
        }
    }

    #[test]
    fn test_answer_message_build_works() {
        let answer: Answer = Answer::default()
            .set_response(_response())
            .set_time(_time())
            .set_thread(_thread());

        assert_eq!(_answer(), answer);

        let expected = r#"{"@id":"testid","@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/questionanswer/1.0/answer","response":"Yes, it's me","~thread":{"received_orders":{},"sender_order":0,"thid":"test_id"},"~timing":{"out_time":"2018-12-13T17:29:34+0000"}}"#;
        assert_eq!(expected, json!(answer.to_a2a_message()).to_string());
    }

    #[test]
    fn test_answer_sign_works() {
        let setup = test_setup::key();

        let answer: Answer = Answer::default()
            .set_response(_response())
            .set_time(_time())
            .sign(&_question(), &setup.key).unwrap()
            .set_thread(_thread());

        let expected_data = ResponseSignature {
            signature: "Zy6vheir8mzbijd5mWSB0NdCEcgt2GofOO3mdjDgA5RtYpgZ6NebGXJzAW9H6kAaOZbhpMjqFsbrFVRh-7P_Ag==".to_string(),
            sig_data: "hiyukYrnayWc6rT3exzsNub_ms8uk55KbKfePSPa6N5vLTEWOv1EyarEQ9ghE4hWVOr6de6liCO-pMJMd67NKA==".to_string(),
            signers: vec!["GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL".to_string()],
            ..ResponseSignature::default()
        };

        assert_eq!(expected_data, answer.response_sig.unwrap());
    }

    #[test]
    fn test_answer_sign_verify_works() {
        let setup = test_setup::key();

        let answer: Answer = Answer::default()
            .set_response(_response())
            .set_time(_time())
            .sign(&_question(), &setup.key).unwrap()
            .set_thread(_thread());

        answer.verify(&setup.key).unwrap();
    }
}