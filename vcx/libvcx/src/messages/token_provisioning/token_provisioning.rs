use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use error::prelude::*;
use messages::agent_utils::{set_config_values, configure_wallet, ComMethod, Config};
use messages::message_type::MessageTypes;
use utils::httpclient;
use settings::ProtocolTypes;
use settings::ProtocolTypes::V2;

pub static VALID_SIGNATURE_ALGORITHMS: [&'static str; 2] = ["SafetyNet", "DeviceCheck"];

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    #[serde(rename = "sponseeId")]
    sponsee_id: String,
    #[serde(rename = "sponsorId")]
    sponsor_id: String,
    #[serde(rename = "pushId")]
    push_id: ComMethod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "sponsorId")]
    sponsor_id: String,
    #[serde(rename = "sponseeId")]
    sponsee_id: String,
    nonce: String,
    timestamp: String,
    sig: String,
    #[serde(rename = "sponsorVerKey")]
    sponsor_vk: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenRequestBuilder {
    sponsee_id: Option<String>,
    sponsor_id: Option<String>,
    push_id: Option<ComMethod>,
    version: Option<ProtocolTypes>,
    agency_did: Option<String>,
}
impl TokenRequestBuilder {
    pub fn build() -> TokenRequestBuilder {
        TokenRequestBuilder {
            sponsee_id: None,
            sponsor_id: None,
            push_id: None,
            version: None,
            agency_did: None,
        }
    }

    pub fn sponsee_id(&mut self, id: &str) -> &mut Self { self.sponsee_id = Some(id.to_string()); self}
    pub fn sponsor_id(&mut self, id: &str) -> &mut Self { self.sponsor_id = Some(id.to_string()); self}
    pub fn push_id(&mut self, push_id: ComMethod) -> &mut Self { self.push_id = Some(push_id); self}
    pub fn version(&mut self, version: ProtocolTypes) -> &mut Self { self.version = Some(version); self}
    pub fn agency_did(&mut self, did: &str) -> &mut Self { self.agency_did = Some(did.to_string()); self}

    pub fn send_secure(&mut self) -> VcxResult<String> {
        trace!("TokenRequestBuilder::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(&response)
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        trace!("TokenRequestBuilder::prepare_request >>>");

        let init_err = |e: &str| VcxError::from_msg(
            VcxErrorKind::CreateWalletBackup,
            format!("TokenRequest expects {} but got None", e)
        );

        let agency_did = self.agency_did.clone().ok_or(init_err("agency_did"))?;
        let message = A2AMessage::Version2(
            A2AMessageV2::TokenRequest(
                TokenRequest {
                    msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(A2AMessageKinds::TokenRequest)),
                    sponsee_id: self.sponsee_id.clone().ok_or(init_err("sponsee_id"))?,
                    sponsor_id: self.sponsor_id.clone().ok_or(init_err("sponsor_id"))?,
                    push_id: self.push_id.clone().ok_or(init_err("push_id"))?,
                }
            )
        );

        trace!("TokenRequestBuilder::prepare_request >>> message: {:?}", secret!(message));

        prepare_message_for_agency(&message, &agency_did, &ProtocolTypes::V3)
    }

    fn parse_response(&self, response: &Vec<u8>) -> VcxResult<String> {
        trace!("TokenRequestBuilder::parse_response >>>");

        let mut response = parse_response_from_agency(response, &ProtocolTypes::V2)?;

        match response.remove(0) {
            A2AMessage::Version1(_) => {
                Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Agency response expected to be of version 2"))
            },
            A2AMessage::Version2(A2AMessageV2::TokenResponse(res)) => Ok(json!(res).to_string()),
            _ => Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Agency response does not match any variant of TokenResponse"))
        }
    }
}

pub fn provision(my_config: Config, sponsee_id: &str, sponsor_id: &str, com_method: ComMethod) -> VcxResult<String> {
    debug!("***Configuring Library");
    set_config_values(&my_config);

    debug!("***Configuring Wallet");
    configure_wallet(&my_config)?;

    debug!("Getting Token");
    let token = TokenRequestBuilder::build()
        .sponsee_id(sponsee_id)
        .sponsor_id(sponsor_id)
        .push_id(com_method)
        .version(V2)
        .agency_did(&my_config.agency_did)
        .send_secure()?;

    Ok(token)
}

#[cfg(test)]
mod tests {
    
    
    
    
    
    
    

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_token_provisioning() {
        cleanup_indy_env();
        init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);

        let seed1 = ::utils::devsetup::create_new_seed();
        let enterprise_wallet_name = format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME);

        let protocol_type = "2.0";
        let config = json!({
            "agency_url": C_AGENCY_ENDPOINT.to_string(),
            "agency_did": C_AGENCY_DID.to_string(),
            "agency_verkey": C_AGENCY_VERKEY.to_string(),
            "wallet_name": enterprise_wallet_name,
            "wallet_key": settings::DEFAULT_WALLET_KEY.to_string(),
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
            "enterprise_seed": seed1,
            "agent_seed": seed1,
            "name": "institution".to_string(),
            "logo": "http://www.logo.com".to_string(),
            "path": constants::GENESIS_PATH.to_string(),
            "protocol_type": protocol_type,
        }).to_string();

        let com_method = ComMethod {
            id: "7b7f97f2".to_string(),
            value: "FCM:Value".to_string(),
            e_type: 1
        };

        provision(parse_config(&config).unwrap(), "123", "456", com_method).unwrap();

        delete_wallet(&enterprise_wallet_name, None, None, None).unwrap();
    }
}

