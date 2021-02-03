use error::{VcxResult, VcxError, VcxErrorKind};
use utils::httpclient::get_status;

#[derive(Serialize, Deserialize)]
struct VerityResponse {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String
}

pub fn health_check() -> VcxResult<()> {
    let result = get_status()?;
    let response = String::from_utf8(result)
        .map_err(|_| VcxError::from_msg(
            VcxErrorKind::InvalidAgencyResponse,
            "Message can't be parsed into String"
        ))?;

    let _: VerityResponse = serde_json::from_str(&response).map_err(|_| {
        VcxError::from_msg(
            VcxErrorKind::InvalidAgencyResponse,
            format!("Unexpected response received. Received response: {}", response)
        )
    })?;

    Ok(())
}