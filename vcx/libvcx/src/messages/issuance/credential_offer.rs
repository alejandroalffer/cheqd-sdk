use error::prelude::*;
use serde_json::Value;

use issuer_credential::PaymentInfo;
use messages::thread::Thread;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CredentialOffer {
    pub msg_type: String,
    pub version: String,
    //vcx version of cred_offer
    pub to_did: String,
    //their_pw_did for this relationship
    pub from_did: String,
    //my_pw_did for this relationship
    pub libindy_offer: String,
    pub cred_def_id: String,
    pub credential_attrs: serde_json::Map<String, serde_json::Value>,
    //promised attributes revealed in credential
    pub schema_seq_no: u32,
    pub claim_name: String,
    pub claim_id: String,
    pub msg_ref_id: Option<String>,
    pub thread_id: Option<String>,
}

pub fn set_cred_offer_ref_message(offer: &str, thread: Option<Thread>, msg_id: &str) -> VcxResult<Vec<Value>> {
    trace!("set_cred_offer_ref_message >>> offer: {:?}, id: {:?}", secret!(offer), msg_id);

    let (mut offer, payment_info) = parse_json_offer(&offer)?;

    offer.msg_ref_id = Some(msg_id.to_owned());
    if let Some(tr) = thread {
        offer.thread_id = tr.thid.clone();
    }

    let mut payload = Vec::new();
    payload.push(json!(offer));
    if let Some(p) = payment_info { payload.push(json!(p)); }

    Ok(payload)
}

pub fn parse_json_offer(offer: &str) -> VcxResult<(CredentialOffer, Option<PaymentInfo>)> {
    trace!("parse_json_offer >>> offer: {:?}", secret!(offer));

    let paid_offer: Value = serde_json::from_str(offer)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, format!("Cannot deserialize Credential Offer: {}", err)))?;

    let mut payment: Option<PaymentInfo> = None;
    let mut offer: Option<CredentialOffer> = None;

    if let Some(i) = paid_offer.as_array() {
        for entry in i.iter() {
            if entry.get("libindy_offer").is_some() {
                offer = Some(serde_json::from_value(entry.clone())
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, format!("Cannot deserialize offer: {}", err)))?);
            }

            if entry.get("payment_addr").is_some() {
                payment = Some(serde_json::from_value(entry.clone())
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize payment address: {}", err)))?);
            }
        }
    }

    let offer = offer
        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, "Message does not contain offer"))?;

    trace!("parse_json_offer <<< offer: {:?}", secret!(offer));

    Ok((offer, payment))
}