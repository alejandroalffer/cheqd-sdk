use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, parse_response_from_agency, prepare_forward_message};
use utils::libindy::{wallet, crypto};
use error::prelude::*;
use messages::agent_utils::{parse_config, set_config_values, configure_wallet, get_final_config};
use serde_json::from_str;
use messages::message_type::MessageTypes;
use messages::thread::Thread;
use utils::uuid::uuid;
use settings;
use utils::httpclient;
use settings::ProtocolTypes;
use messages::token_provisioning::token_provisioning::VALID_SIGNATURE_ALGORITHMS;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionToken {
    #[serde(rename = "sponseeId")]
    sponsee_id: String,
    #[serde(rename = "sponsorId")]
    sponsor_id: String,
    nonce: String,
    timestamp: String,
    sig: String,
    #[serde(rename = "sponsorVerKey")]
    sponsor_vk: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attestationAlgorithm")]
    attestation_algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attestationData")]
    attestation_data: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RequesterKeys {
    #[serde(rename = "fromDID")]
    pub my_did: String,
    #[serde(rename = "fromVerKey")]
    pub my_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionAgent {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    #[serde(rename = "provisionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<ProvisionToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requesterKeys")]
    requester_keys: Option<RequesterKeys>,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentCreated {
    #[serde(rename = "selfDID")]
    pub self_did: String,
    #[serde(rename = "agentVerKey")]
    pub agent_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProblemReport {
    pub err: String,
}

impl ProvisionToken {
    pub fn validate(&self) -> VcxResult<()> {
        match (self.attestation_data.as_ref(), self.attestation_algorithm.as_ref()) {
            (Some(_), None) | (None, Some(_)) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidConfiguration, "signature and algorithm must be either passed or skipped together"));
            },
            (Some(_), Some(algorithm)) if !VALID_SIGNATURE_ALGORITHMS.contains(&algorithm.as_str()) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidConfiguration,
                                              format!("unexpected signature algorithm was passed: {:?}. expected: {:?}", algorithm, VALID_SIGNATURE_ALGORITHMS)));
            }
            _ => Ok(())
        }
    }
}

impl ProvisionAgent {
    pub fn build(token: Option<ProvisionToken>, keys: Option<RequesterKeys>) -> ProvisionAgent {
        ProvisionAgent {
            msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(A2AMessageKinds::ProvisionAgent)),
            requester_keys: keys,
            token,
            thread: Thread::new().set_thid(uuid())
        }
    }
}
pub fn provision(config: &str, token: &str) -> VcxResult<String> {
    trace!("connect_register_provision >>> config: {:?}", secret!(config));
    let my_config = parse_config(config)?;
    let token: ProvisionToken = from_str(token).map_err(|err| VcxError::from_msg(
        VcxErrorKind::InvalidProvisioningToken,
        format!("Cannot parse config: {}", err)
    ))?;
    token.validate()?;

    debug!("***Configuring Library");
    set_config_values(&my_config);

    debug!("***Configuring Wallet");
    let (my_did, my_vk, wallet_name) = configure_wallet(&my_config)?;

    debug!("Connecting to Agency");
    let (agent_did, agent_vk) = create_agent(&my_did, &my_vk, token)?;

    debug!("Building config");
    let config = get_final_config(&my_did, &my_vk, &agent_did, &agent_vk, &wallet_name, &my_config)?;

    wallet::close_wallet()?;

    Ok(config)
}

pub fn create_agent(my_did: &str, my_vk: &str, token: ProvisionToken) -> VcxResult<(String, String)> {
    /* STEP 1 - CREATE AGENT */
    debug!("Creating an agent");

    let keys = RequesterKeys {my_did: my_did.to_string(), my_vk: my_vk.to_string()};
    let message = A2AMessage::Version2(
        A2AMessageV2::ProvisionAgent(
            ProvisionAgent::build(Some(token), Some(keys))
        )
    );

    let mut response = provisioning_v0_7_send_message_to_agency(&message, &my_vk)?;

    let response: AgentCreated =
        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::AgentCreated(resp)) => resp,
            A2AMessage::Version2(A2AMessageV2::ProblemReport(resp)) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidProvisioningToken, format!("provisioning failed: {:?}", resp)))
            },
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Agency response does not match any variant of AgentCreated"))
        };

    Ok((response.self_did, response.agent_vk))
}

pub fn provisioning_v0_7_send_message_to_agency(message: &A2AMessage, from_vk: &str) -> VcxResult<Vec<A2AMessage>> {
    let data = provisioning_v0_7_pack_for_agency(message, from_vk)?;

    let response = httpclient::post_u8(&data)?;

    parse_response_from_agency(&response, &ProtocolTypes::V3)
}

pub fn provisioning_v0_7_pack_for_agency(message: &A2AMessage, from_vk: &str) -> VcxResult<Vec<u8>> {
    trace!("provisioning_v0_7_pack_for_agency >>>");

    /*
    * 1. encodes message using provided verkey and sets Agency verkey as recipient
    * 2. wraps encoded message into Forward to Agency DID (itself)
    */

    let agency_did = settings::get_config_value(settings::CONFIG_AGENCY_DID)?;
    let agency_verkey = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;

    let message = json!(message).to_string();
    let receiver_keys =  json!(vec![&agency_verkey]).to_string();

    let message = crypto::pack_message(Some(from_vk), &receiver_keys, message.as_bytes())?;

    let forward = prepare_forward_message(message, &agency_did, ProtocolTypes::V3)?;

    trace!("provisioning_v0_7_pack_for_agency <<<");
    Ok(forward)
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings;
    use utils::constants;
    use utils::devsetup::{C_AGENCY_DID, C_AGENCY_VERKEY, C_AGENCY_ENDPOINT, cleanup_indy_env, sign_provision_token};
    use utils::plugins::init_plugin;

    fn get_provisioning_inputs(time: Option<String>, seed: Option<String>) -> (String, String, String) {
        let enterprise_wallet_name = format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME);
        wallet::delete_wallet(&enterprise_wallet_name, None, None, None).err();

        let id = "id";
        let sponsor_id = "evernym-test-sponsorabc123";
        let nonce = "nonce";
        let time = time.unwrap_or(chrono::offset::Utc::now().to_rfc3339());
        let seed = seed.unwrap_or("000000000000000000000000Trustee1".to_string());
        println!("Time: {:?}", time);
        wallet::init_wallet(&enterprise_wallet_name, None, None, None).unwrap();
        let keys = ::utils::libindy::crypto::create_key(Some(&seed)).unwrap();
        let encoded_val = sign_provision_token(&keys, &nonce, &time, &id, &sponsor_id);
        let seed1 = ::utils::devsetup::create_new_seed();
        wallet::close_wallet().err();
        wallet::delete_wallet(&enterprise_wallet_name, None, None, None).err();

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

        let token = json!( {
            "sponseeId": id.to_string(),
            "sponsorId": sponsor_id.to_string(),
            "nonce": nonce.to_string(),
            "timestamp": time.to_string(),
            "sig": encoded_val,
            "sponsorVerKey": keys.to_string()
        }).to_string();
        (enterprise_wallet_name, config, token)
    }

    #[ignore]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_agent_provisioning_0_7() {
        cleanup_indy_env();
        init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);

        let (wallet_name, config, token) = get_provisioning_inputs(None, None);
        let _enterprise_config = provision(&config, &token).unwrap();

        wallet::delete_wallet(&wallet_name, None, None, None).unwrap();
    }

    #[ignore]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_agent_provisioning_0_7_fails_with_expired_time() {
        cleanup_indy_env();
        init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);

        let new_time = "2020-03-20T13:00:00+00:00";
        let (wallet_name, config, token) = get_provisioning_inputs(Some(new_time.to_string()), None);
        assert!(provision(&config, &token).is_err());

        wallet::delete_wallet(&wallet_name, None, None, None).unwrap();
    }

    #[ignore]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_agent_provisioning_0_7_fails_with_invalid_sig() {
        cleanup_indy_env();
        init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);

        let (wallet_name, config, token) = get_provisioning_inputs(None, Some("000000000000000000000000Truste22".to_string()));
        assert!(provision(&config, &token).is_err());

        wallet::delete_wallet(&wallet_name, None, None, None).unwrap();
    }
}

