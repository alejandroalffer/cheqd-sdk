use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialOffer {
    pub schema_id: String,
    pub cred_def_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub cred_rev_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Credential {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub values: CredentialValues,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialValues(pub HashMap<String, AttributeValues>);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct AttributeValues {
    pub raw: String,
    pub encoded: String,
}
