#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CredentialMessage {
    pub libindy_cred: String,
    pub rev_reg_def_json: String,
    pub cred_def_id: String,
    pub msg_type: String,
    pub claim_offer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_revoc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoc_reg_delta_json: Option<String>,
    pub version: String,
    pub from_did: String,
}
