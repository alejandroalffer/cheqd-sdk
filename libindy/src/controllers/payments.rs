use std::{
    collections::HashMap,
    sync::Arc,
    string::String,
    vec::Vec
};

use hex;

use serde_json;

use indy_api_types::errors::prelude::*;
use crate::{
    services::{
        CryptoService,
        LedgerService,
        payments::{
            PaymentsMethodCBs, PaymentsService, RequesterInfo, Fees
        }
    },
    domain::{
        ledger::auth_rule::AuthRule,
        crypto::did::DidValue
    }
};
use indy_wallet::{RecordOptions, WalletService};
use indy_api_types::WalletHandle;

pub struct PaymentsController {
    payments_service:Arc<PaymentsService>,
    wallet_service:Arc<WalletService>,
    crypto_service:Arc<CryptoService>,
    ledger_service:Arc<LedgerService>,
}

impl PaymentsController {
    pub(crate) fn new(payments_service:Arc<PaymentsService>, wallet_service:Arc<WalletService>, crypto_service:Arc<CryptoService>, ledger_service:Arc<LedgerService>) -> PaymentsController {
        PaymentsController {
            payments_service,
            wallet_service,
            crypto_service,
            ledger_service,
        }
    }

    fn register_method(&self, type_: String, methods: PaymentsMethodCBs) -> IndyResult<()> {
        trace!("register_method > type_ {:?} methods {:?}", type_, methods);

        self.payments_service.register_payment_method(&type_, methods);
        let res = Ok(());

        trace!("register_method << res {:?}", res);

        res
    }

    pub(crate) async fn create_address(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        config: String
    ) -> IndyResult<String> {
        trace!("create_address > wallet_handle {:?} type_ {:?} config {:?}", wallet_handle, type_, config);

        self.wallet_service.check(wallet_handle).await.map_err(map_err_err!())?;

        let res = self
            .payments_service
            .create_address(wallet_handle, &type_, &config)
            .await?;

        //TODO: think about deleting payment_address on wallet save failure
        self.wallet_service.add_record(wallet_handle, &self.wallet_service.add_prefix("PaymentAddress"), &res, &res, &HashMap::new()).await?;

        trace!("create_address < {}", res);
        Ok(res)
    }

    pub(crate) async fn list_addresses(&self, wallet_handle: WalletHandle) -> IndyResult<String> {
        trace!("list_addresses > wallet_handle {:?}", wallet_handle);

        let mut search = self.wallet_service.search_records(wallet_handle, &self.wallet_service.add_prefix("PaymentAddress"), "{}", &RecordOptions::id_value()).await?;

        let mut list_addresses: Vec<String> = Vec::new();

        while let Ok(Some(payment_address)) = search.fetch_next_record().await {
            let value = payment_address.get_value().ok_or(err_msg(IndyErrorKind::InvalidState, "Record value not found"))?;
            list_addresses.push(value.to_string());
        }

        let json_string_res = serde_json::to_string(&list_addresses)
            .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize List of Payment Addresses");

        trace!("list_addresses <");
        json_string_res
    }

    pub(crate) async fn add_request_fees(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        req: String,
        inputs: String,
        outputs: String,
        extra: Option<String>
    ) -> IndyResult<(String, String)> {
        trace!("add_request_fees > wallet_handle {:?} submitter_did {:?} req {:?} inputs {:?} outputs {:?} extra {:?}",
               wallet_handle, submitter_did, req, inputs, outputs, extra);

        self.crypto_service.validate_opt_did(submitter_did.as_ref()).map_err(map_err_err!())?;

        let method_from_inputs = self.payments_service.parse_method_from_inputs(&inputs);

        let method = if outputs == "[]" {
            method_from_inputs?
        } else {
            let method_from_outputs = self.payments_service.parse_method_from_outputs(&outputs);
            PaymentsController::_merge_parse_result(method_from_inputs, method_from_outputs)?
        };

        let req = self
            .payments_service
            .add_request_fees(
                &method,
                wallet_handle,
                submitter_did.as_ref(),
                &req,
                &inputs,
                &outputs,
                extra.as_deref()
            ).await?;

        trace!("add_request_fees <");
        Ok((req, method))
    }

    pub(crate) async fn parse_response_with_fees(
        &self,
        type_: String,
        response: String
    ) -> IndyResult<String> {
        trace!("parse_response_with_fees > type_ {:?} response {:?}", type_, response);

        let res = self
            .payments_service
            .parse_response_with_fees(&type_, &response)
            .await;

        trace!("parse_response_with_fees < {:?}", res);
        res
    }

    pub(crate) async fn build_get_payment_sources_request(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        payment_address: String,
        next: Option<i64>
    ) -> IndyResult<(String, String)> {
        trace!("build_get_payment_sources_request > wallet_handle {:?} \
               submitter_did {:?} payment_address {:?}",
               wallet_handle, submitter_did, payment_address);

        self
            .crypto_service
            .validate_opt_did(submitter_did.as_ref())
            .map_err(map_err_err!())?;

        let method = self
            .payments_service
            .parse_method_from_payment_address(&payment_address)?;
        let req = self
            .payments_service
            .build_get_payment_sources_request(
                &method,
                wallet_handle,
                submitter_did.as_ref(),
                &payment_address,
                next
            )
            .await?;

        trace!("build_get_payment_sources_request < req {} method {}", req, method);
        Ok((req, method))
    }

    pub(crate) async fn parse_get_payment_sources_response(
        &self,
        type_: String,
        response: String
    ) -> IndyResult<(String, i64)> {
        trace!("parse_get_payment_sources_response > response {:?}", response);

        let res = self
            .payments_service
            .parse_get_payment_sources_response(&type_, &response)
            .await;

        trace!("parse_get_payment_sources_response < {:?}", res);
        res
    }

    pub(crate) async fn build_payment_req(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        inputs: String,
        outputs: String,
        extra: Option<String>
    ) -> IndyResult<(String, String)> {
        trace!("build_payment_req > wallet_handle {:?} submitter_did {:?} \
               inputs {:?} outputs {:?} extra {:?}",
               wallet_handle, submitter_did, inputs, outputs, extra);

        self.crypto_service.validate_opt_did(submitter_did.as_ref()).map_err(map_err_err!())?;

        let method_from_inputs = self
            .payments_service
            .parse_method_from_inputs(&inputs);
        let method_from_outputs = self
            .payments_service
            .parse_method_from_outputs(&outputs);
        let method =
            PaymentsController::_merge_parse_result(method_from_inputs, method_from_outputs)?;

        let req = self
            .payments_service
            .build_payment_req(
                &method,
                wallet_handle,
                submitter_did.as_ref(),
                &inputs,
                &outputs,
                extra.as_deref()).await?;

        trace!("build_payment_req <");
        Ok((req, method))
    }

    fn append_txn_author_agreement_acceptance_to_extra(&self,
                                                       extra: Option<String>,
                                                       text: Option<String>,
                                                       version: Option<String>,
                                                       taa_digest: Option<String>,
                                                       mechanism: String,
                                                       time: u64) -> IndyResult<String> {
        debug!("append_txn_author_agreement_acceptance_to_extra > \
               extra {:?} text {:?} version {:?} taa_digest {:?} \
               mechanism {:?} time {:?}", extra, text, version,
               taa_digest, mechanism, time);

        let mut extra: serde_json::Value = serde_json::from_str(&extra.unwrap_or("{}".to_string()))
            .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Cannot deserialize extra {:?}", err)))?;

        let acceptance_data = self
            .ledger_service
            .prepare_acceptance_data(
                text.as_deref(),
                version.as_deref(),
                taa_digest.as_deref(),
                &mechanism,
                time
            )?;

        extra["taaAcceptance"] = serde_json::to_value(acceptance_data)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize author agreement acceptance data")?;

        let res: String = extra.to_string();

        debug!("append_txn_author_agreement_acceptance_to_extra < res {:?}", res);

        Ok(res)
    }

    pub(crate) async fn parse_payment_response(
        &self,
        payment_method: String,
        response: String
    ) -> IndyResult<String> {
        trace!("parse_payment_response > response {:?}", response);

        let res = self
            .payments_service
            .parse_payment_response(&payment_method, &response)
            .await;

        trace!("parse_payment_response < {:?}", res);
        res
    }

    pub(crate) async fn build_mint_req(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        outputs: String,
        extra: Option<String>
    ) -> IndyResult<(String, String)> {
        trace!("build_mint_req > wallet_handle {:?} submitter_did {:?} \
               outputs {:?} extra {:?}", wallet_handle, submitter_did,
               outputs, extra);

        self.crypto_service.validate_opt_did(submitter_did.as_ref()).map_err(map_err_err!())?;

        let type_ = self.payments_service.parse_method_from_outputs(&outputs)?;
        let req = self
            .payments_service
            .build_mint_req(
                &type_,
                wallet_handle,
                submitter_did.as_ref(),
                &outputs,
                extra.as_deref()
            )
            .await?;

        trace!("build_mint_req < req {} method {}", req, type_);
        Ok((req, type_))
    }

    pub(crate) async fn build_set_txn_fees_req(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        type_: String,
        fees: String
    ) -> IndyResult<String> {
        trace!("build_set_txn_fees_req > wallet_handle {:?} submitter_did {:?} \
               type_ {:?} fees {:?}", wallet_handle, submitter_did, type_, fees);

        self.crypto_service.validate_opt_did(submitter_did.as_ref()).map_err(map_err_err!())?;
        serde_json::from_str::<HashMap<String, i64>>(&fees).map_err(|err| {
            error!("Cannot deserialize Fees {:?}", err);
            err.to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize Fees")
        })?;

        let res = self
            .payments_service
            .build_set_txn_fees_req(
                &type_,
                wallet_handle,
                submitter_did.as_ref(),
                &fees
            )
            .await;

        trace!("build_set_txn_fees_req < {:?}", res);
        res
    }

    pub(crate) async fn build_get_txn_fees_req(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        type_: String
    ) -> IndyResult<String> {
        trace!("build_get_txn_fees_req > wallet_handle {:?} submitter_did {:?} \
               type_ {:?}", wallet_handle, submitter_did, type_);

        self.crypto_service
            .validate_opt_did(submitter_did.as_ref())
            .map_err(map_err_err!())?;

        let res = self
            .payments_service
            .build_get_txn_fees_req(
                &type_,
                wallet_handle,
                submitter_did.as_ref()
            )
            .await;

        trace!("build_get_txn_fees_req < {:?}", res);
        res
    }

    pub(crate) async fn parse_get_txn_fees_response(
        &self,
        type_: String,
        response: String
    ) -> IndyResult<String> {
        trace!("parse_get_txn_fees_response > response {:?}", response);

        let res = self
            .payments_service
            .parse_get_txn_fees_response(&type_, &response)
            .await;

        trace!("parse_get_txn_fees_response < {:?}", res);
        res
    }

    pub(crate) async fn build_verify_payment_request(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: Option<DidValue>,
        receipt: String
    ) -> IndyResult<(String, String)> {
        trace!("build_verify_payment_request > wallet_handle {:?} submitter_did {:?} \
               receipt {:?}", wallet_handle, submitter_did, receipt);

        self.crypto_service
            .validate_opt_did(submitter_did.as_ref())
            .map_err(map_err_err!())?;

        let method = self
            .payments_service
            .parse_method_from_payment_address(&receipt)?;
        let req = self
            .payments_service
            .build_verify_payment_req(
                &method,
                wallet_handle,
                submitter_did.as_ref(),
                &receipt
            )
            .await?;

        trace!("build_verify_payment_request <");
        Ok((req, method))
    }

    pub(crate) async fn parse_verify_payment_response(&self, type_: String, resp_json: String) -> IndyResult<String> {
        trace!("parse_verify_payment_response > response {:?}", resp_json);

        let res = self
            .payments_service
            .parse_verify_payment_response(&type_, &resp_json)
            .await;

        trace!("parse_verify_payment_response < {:?}", res);
        res
    }

    pub(crate) async fn sign_with_address<'a>(
        &'a self,
        wallet_handle: WalletHandle,
        address: String,
        message: &'a [u8]
    ) -> IndyResult<Vec<u8>> {
        trace!("sign_with_address > address {:?} message {:?}", address, hex::encode(message));

        let method = self
            .payments_service
            .parse_method_from_payment_address(&address)?;

        self.payments_service
            .sign_with_address(
                &method,
                wallet_handle,
                &address,
                message
            )
            .await
    }

    pub(crate) async fn verify_with_address<'a>(
        &'a self,
        address: String,
        message: &'a [u8],
        signature: &'a [u8]
    ) -> IndyResult<bool> {
        trace!("sign_with_address > address {:?} message {:?} \
                signature {:?}", address, hex::encode(message),
               hex::encode(signature));

        let method = self.payments_service.parse_method_from_payment_address(&address)?;

        self.payments_service.verify_with_address(&method, &address, message, signature).await
    }


    // HELPERS

    fn _merge_parse_result(method_from_inputs: IndyResult<String>, method_from_outputs: IndyResult<String>) -> IndyResult<String> {
        match (method_from_inputs, method_from_outputs) {
            (Err(err), _) | (_, Err(err)) => Err(err),
            (Ok(ref mth1), Ok(ref mth2)) if mth1 != mth2 => {
                error!("Different payment method in inputs and outputs");
                Err(err_msg(IndyErrorKind::IncompatiblePaymentMethods, "Different payment method in inputs and outputs"))
            }
            (Ok(mth1), Ok(_)) => Ok(mth1)
        }
    }

    pub fn get_request_info(&self, get_auth_rule_response_json: &str, requester_info: RequesterInfo, fees: &Fees) -> IndyResult<String> {
        trace!("get_request_info > get_auth_rule_response_json {:?} requester_info {:?} fees {:?}", get_auth_rule_response_json, requester_info, fees);

        let auth_rule = self._parse_get_auth_rule_response(get_auth_rule_response_json)?;

        let req_info = self.payments_service.get_request_info_with_min_price(&auth_rule.constraint, &requester_info, &fees)?;

        let res = serde_json::to_string(&req_info)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RequestInfo")?;

        trace!("get_request_info < {:?}", res);

        Ok(res)
    }

    fn _parse_get_auth_rule_response(&self, get_auth_rule_response_json: &str) -> IndyResult<AuthRule> {
        trace!("_parse_get_auth_rule_response > get_auth_rule_response_json {:?}", get_auth_rule_response_json);

        let mut auth_rules: Vec<AuthRule> = self.ledger_service.parse_get_auth_rule_response(get_auth_rule_response_json)?;

        if auth_rules.len() != 1 {
            return Err(IndyError::from_msg(IndyErrorKind::InvalidTransaction, "GetAuthRule response must contain one auth rule"));
        }

        let res = auth_rules.pop().unwrap();

        trace!("_parse_get_auth_rule_response < {:?}", res);

        Ok(res)
    }
}
