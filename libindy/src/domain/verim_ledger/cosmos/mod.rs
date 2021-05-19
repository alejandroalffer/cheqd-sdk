mod base;
mod crypto;
mod tx;

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::cosmos::tx::signing::SignMode;
    use crate::domain::verim_ledger::cosmos::tx::{ModeInfo, SignerInfo};
    use serde_json::Value;

    #[test]
    fn test_auth_info_to_json_conversion() {
        let signer_info = SignerInfo::new(
            Some(prost_types::Any {
                type_url: "type url".to_string(),
                value: Vec::new(),
            }),
            Some(ModeInfo::Single(SignMode::Direct)),
            123,
        );

        let json = serde_json::to_value(&signer_info).unwrap();

        let expected: Value = serde_json::from_str(
            r#""auth_info": {
            "signer_infos": [
              {
                "public_key": {
                  "@type": "/cosmos.crypto.secp256k1.PubKey",
                  "key": "A8YkdMdwy1ZpOJsv8cY4r/g09nAJCmcibmEyzg60kgq7"
                },
                "mode_info": {
                  "single": {
                    "mode": "SIGN_MODE_DIRECT"
                  }
                },
                "sequence": "0"
              }
            ],
            "fee": {
              "amount": [],
              "gas_limit": "200000",
              "payer": "",
              "granter": ""
            }
          }"#,
        )
        .unwrap();

        assert_eq!(json, expected);
    }
}
