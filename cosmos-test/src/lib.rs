use cosmos_sdk::crypto::secp256k1::SigningKey;
use cosmos_sdk::crypto::PublicKey;
use cosmos_sdk::tendermint::chain::Id;
use cosmos_sdk::tx::{Msg, Raw};
use cosmos_sdk::{
    bank::MsgSend,
    crypto::secp256k1,
    tx::{self, Fee, MsgType, SignDoc, SignerInfo},
    AccountId, Coin,
};

fn build_tx(
    sender_private_key: &SigningKey,
    sender_public_key: PublicKey,
    msgs: Vec<Msg>,
    chain_id: &Id,
    account_number: u64,
    sequence_number: u64,
    gas_limit: u64,
    fee_limit: Coin,
    timeout_height: u16,
    memo: &str,
) -> cosmos_sdk::Result<Raw> {
    let tx_body = tx::Body::new(msgs, memo, timeout_height);

    let signer_info = SignerInfo::single_direct(Some(sender_public_key), sequence_number);

    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(fee_limit, gas_limit));

    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

    let tx_signed = sign_doc.sign(&sender_private_key)?;

    Ok(tx_signed)
}

fn build_msg_bank_send(
    sender_account_id: AccountId,
    recipient_account_id: AccountId,
    amount: u64,
    denom: &str,
) -> cosmos_sdk::Result<MsgSend> {
    let amount = Coin {
        amount: amount.into(),
        denom: denom.parse()?,
    };

    let msg_send = MsgSend {
        from_address: sender_account_id,
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    };

    Ok(msg_send)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_from_seed_works() {
        let mnemonic = "same fury judge fruit assault goose garbage bean stumble sand pen brisk scrub return general alien delay dilemma link stamp project derive awesome action";
        let address = "cosmos1ep7nmpzhd848tg5mqt02r4fpvuldpnffarqp6v";
        let pub_key = "cosmospub1addwnpepqwjxkrkms4e27ursuyxnkd6wcyrp40aw3xkmmnramv5vs6w62ganc6zzggw";

        // We need `from_mnemonic` here
        let sender_private_key = secp256k1::SigningKey::random();
        let sender_public_key = sender_private_key.public_key();
        let sender_account_id = sender_public_key.account_id("cosmos").unwrap();

        assert_eq!(sender_account_id, address.parse().unwrap());
    }

    #[test]
    fn test_build_tx_works() {
        // Build msg
        let sender_private_key = secp256k1::SigningKey::random();
        let sender_public_key = sender_private_key.public_key();
        let sender_account_id = sender_public_key.account_id("cosmos").unwrap();

        let recipient_account_id =
            "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".parse::<AccountId>().unwrap();

        let msg_send = build_msg_bank_send(sender_account_id, recipient_account_id, 1000, "stake").unwrap();

        // Build tx
        let chain_id = "cosmoshub-4".parse().unwrap();
        let account_number = 1;
        let sequence_number = 0;
        let timeout_height = 9001u16;
        let memo = "example memo";

        let gas = 100_000;
        let fee = Coin {
            denom: "stake".parse().unwrap(),
            amount: 1000u64.into(),
        };

        let tx = build_tx(
            &sender_private_key,
            sender_public_key,
            vec![msg_send.to_msg().unwrap()],
            &chain_id,
            account_number,
            sequence_number,
            gas,
            fee,
            timeout_height,
            memo,
        ).unwrap();

        assert_ne!(tx.to_bytes().unwrap().len(), 0);
    }
}
