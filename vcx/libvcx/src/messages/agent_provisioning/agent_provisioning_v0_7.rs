use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds};
use messages::message_type::{MessageTypes, MessageTypeV2};
use utils::libindy::wallet;
use error::prelude::*;
use messages::agent_utils::{parse_config, set_config_values, configure_wallet, get_final_config, connect_v2, send_message_to_agency, CreateAgent, AgentCreated};
use serde_json::from_str;


#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionToken {
    id: String,
    sponsor: String,
    nonce: String,
    timestamp: String,
    sig: String,
    #[serde(rename = "sponsorVerKey")]
    sponsor_vk: String,
}

pub fn provision(config: &str, token: &str) -> VcxResult<String> {
    trace!("connect_register_provision >>> config: {:?}", config);
    let my_config = parse_config(config)?;
    let token: ProvisionToken = from_str(token).map_err(|err| VcxError::from_msg(
        VcxErrorKind::InvalidProvisioningToken,
        format!("Cannot parse config: {}", err)
    ))?;

    trace!("***Configuring Library");
    set_config_values(&my_config);

    trace!("***Configuring Wallet");
    let (my_did, my_vk, wallet_name) = configure_wallet(&my_config)?;

    trace!("Connecting to Agency");
    let (agent_did, agent_vk) = create_agent(&my_did, &my_vk, &my_config.agency_did, token)?;
    wallet::close_wallet()?;

    get_final_config(&my_did, &my_vk, &agent_did, &agent_vk, &wallet_name, &my_config)
}

pub fn create_agent(my_did: &str, my_vk: &str, agency_did: &str, token: ProvisionToken) -> VcxResult<(String, String)> {
    debug!("Connecting with Evernym's Agency");
    println!("Connecting with Evernym's Agency");
    let (agency_pw_did, _) = connect_v2(my_did, my_vk, agency_did)?;

    /* STEP 2 - CREATE AGENT */
    debug!("Creating an agent");
    println!("Creating an agent");
    let mut ca = CreateAgent::build(Some(my_did.to_string()), Some(my_vk.to_string()), Some(token));
    if let MessageTypes::MessageTypeV2(msg) = &ca.msg_type {
        ca.msg_type = MessageTypes::MessageTypeV2(MessageTypeV2 {
            did: msg.did.to_string(),
            family: msg.family.to_owned(),
            version: "0.7".to_string(),
            type_: msg.type_.to_string(),
        });
    }
    let message = A2AMessage::Version2(A2AMessageV2::CreateAgent(ca));

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

    let response: AgentCreated =
        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::AgentCreated(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of AgentCreated"))
        };

    println!("response: {:?}", response);
    Ok((response.from_did, response.from_vk))
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings;

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_agent_provisioning_0_7() {
        use std::time::{Duration, SystemTime};
        use chrono::prelude::*;
        use utils::constants;
        use utils::devsetup::tests::{C_AGENCY_DID, C_AGENCY_VERKEY, C_AGENCY_ENDPOINT};
        ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, ::settings::DEFAULT_WALLET_NAME));
        ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::CONSUMER_PREFIX, ::settings::DEFAULT_WALLET_NAME));
        ::utils::libindy::pool::tests::delete_test_pool();
        ::utils::devsetup::tests::init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);

        let id = "id";
        let sponsor = "evernym-test-sponsor";
        let nonce = "nonce";
        let time = chrono::offset::Utc::now().to_rfc3339();
        println!("Time: {:?}", time);
        let enterprise_wallet_name = format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME);
        wallet::init_wallet(&enterprise_wallet_name, None, None, None).unwrap();
        let keys = ::utils::libindy::crypto::create_key(Some("000000000000000000000000Trustee1")).unwrap();
        let sig = ::utils::libindy::crypto::sign(&keys, &(format!("{}{}{}", nonce, time, id)).as_bytes()).unwrap();
        let encoded_val = base64::encode(&sig);
        let seed1 = ::utils::devsetup::tests::create_new_seed();
        wallet::close_wallet();
        wallet::delete_wallet(&enterprise_wallet_name, None, None, None);

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
            "id": id.to_string(),
            "sponsor": sponsor.to_string(),
            "nonce": nonce.to_string(),
            "timestamp": time.to_string(),
            "sig": encoded_val,
            "sponsorVerKey": keys.to_string()
        }).to_string();

        let enterprise_config = provision(&config, &token).unwrap();

        wallet::delete_wallet(&enterprise_wallet_name, None, None, None).unwrap();
    }
}

