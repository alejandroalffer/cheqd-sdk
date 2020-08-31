use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds};
use utils::libindy::wallet;
use error::prelude::*;
use messages::agent_utils::{parse_config, set_config_values, configure_wallet, get_final_config, connect_v2, send_message_to_agency, update_agent_profile};
use serde_json::from_str;
use messages::message_type::MessageTypes;
use settings::{config_str_to_bool, ProtocolTypes, CONFIG_USE_PUBLIC_DID};
use settings;


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
    requester_keys: Option<RequesterKeys>
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

impl ProvisionAgent {
    pub fn build(token: Option<ProvisionToken>, keys: Option<RequesterKeys>) -> ProvisionAgent {
        ProvisionAgent {
            msg_type: MessageTypes::build(A2AMessageKinds::ProvisionAgent),
            requester_keys: keys,
            token,
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

    debug!("***Configuring Library");
    set_config_values(&my_config);

    debug!("***Configuring Wallet");
    let (my_did, my_vk, wallet_name) = configure_wallet(&my_config)?;

    debug!("Connecting to Agency");
    let (agent_did, agent_vk) = create_agent(&my_did, &my_vk, &my_config.agency_did, token)?;


    /* Update Agent Info */
    let mut public_did: Option<String> = None;
    if config_str_to_bool(CONFIG_USE_PUBLIC_DID).unwrap_or(false) {
        public_did = Some(settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?);
    };
    update_agent_profile(&agent_did,
                         &public_did,
                         ProtocolTypes::V2)?;

    wallet::close_wallet()?;

    debug!("Building config");
    get_final_config(&my_did, &my_vk, &agent_did, &agent_vk, &wallet_name, &my_config)
}

pub fn create_agent(my_did: &str, my_vk: &str, agency_did: &str, token: ProvisionToken) -> VcxResult<(String, String)> {
    debug!("Connecting with Evernym's Agency");
    let (agency_pw_did, _) = connect_v2(my_did, my_vk, agency_did)?;

    /* STEP 2 - CREATE AGENT */
    debug!("Creating an agent");

    let keys = RequesterKeys {my_did: my_did.to_string(), my_vk: my_vk.to_string()};
    let message = A2AMessage::Version2(
        A2AMessageV2::ProvisionAgent(
            ProvisionAgent::build(Some(token), Some(keys))
        )
    );

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

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
            "use_public_did": false
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

