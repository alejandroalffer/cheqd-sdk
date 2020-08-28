use utils::devsetup::{SetupEmpty, create_new_seed, AGENCY_ENDPOINT, AGENCY_DID, AGENCY_VERKEY, config_with_wallet_handle};
use utils::constants;
use ::{settings, utils};
use utils::libindy::wallet;
use utils::libindy::pool::tests::open_test_pool;
use connection::{create_connection, connect, get_invite_details, update_state};
use std::time::{SystemTime, Duration};
use std::thread;
use api::vcx::vcx_shutdown;

const CONNECTION_CNT: usize = 10;
const UPDATE_CNT: usize = 10;

fn _setup() {
    println!("Started");

    let _setup = SetupEmpty::init();

    let wallet_name = format!("{}_{}", constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME);
    let config = json!({
        "agency_url": AGENCY_ENDPOINT.to_string(),
        "agency_did": AGENCY_DID.to_string(),
        "agency_verkey": AGENCY_VERKEY.to_string(),
        "wallet_name": wallet_name,
        "wallet_key": settings::DEFAULT_WALLET_KEY.to_string(),
        "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        "enterprise_seed": create_new_seed(),
        "agent_seed": create_new_seed(),
        "name": "institution".to_string(),
        "logo": "http://www.logo.com".to_string(),
        "path": constants::GENESIS_PATH.to_string(),
        "protocol_type": "1.0"
    });

    let enterprise_config = ::messages::agent_utils::connect_register_provision(&config.to_string()).unwrap();
    let enterprise_config = config_with_wallet_handle(&wallet_name, &enterprise_config);
    settings::process_config_string(&enterprise_config, true);

    settings::set_config_value(settings::CONFIG_GENESIS_PATH, utils::get_temp_dir_path(settings::DEFAULT_GENESIS_PATH).to_str().unwrap());
    open_test_pool();
}

#[test]
fn test_connections() {
    _setup();

    let connections = std::env::var("CONNECTION_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(CONNECTION_CNT);
    let polling_counts = std::env::var("UPDATE_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(UPDATE_CNT);

    let mut threads = vec![];

    for _ in 0..connections {
        let handle = thread::spawn(move || {
            let id = uuid::Uuid::new_v4().to_string();

            println!("--- Create connection \"{}\"---", id);
            let start_time = SystemTime::now();
            let connection_handle = create_connection(&id).unwrap();
            println!("--- Create connection \"{}\" Time: {:?}", id, SystemTime::now().duration_since(start_time).unwrap());

            println!("--- Connection connect--- \"{}\"---", id);
            let start_time = SystemTime::now();
            connect(connection_handle, None).unwrap();
            println!("--- Connection connect \"{}\" Time: {:?}", id, SystemTime::now().duration_since(start_time).unwrap());

            let details = get_invite_details(connection_handle, false).unwrap();
            println!("--- Connection Invite \"{}\" --- {:?}", id, details);

            for i in 0..polling_counts {
                println!("--- Connection update state \"{}\" ---", id);
                let start_time = SystemTime::now();
                update_state(connection_handle, None).unwrap();
                println!("--- Connection update state \"{}\" Time: {:?}", id, SystemTime::now().duration_since(start_time).unwrap());
                thread::sleep(Duration::from_secs(4));
            }

            println!("----------------------------------------------")
        });
        threads.push(handle);
        thread::sleep(Duration::from_secs(2));
    }

    for thread_ in threads {
        thread_.join().unwrap();
    }

    vcx_shutdown(true);
}