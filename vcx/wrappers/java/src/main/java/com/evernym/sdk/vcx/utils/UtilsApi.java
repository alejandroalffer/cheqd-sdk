package com.evernym.sdk.vcx.utils;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;


/**
 * Created by abdussami on 17/05/18.
 */

public class UtilsApi extends VcxJava.API {
    private static final Logger logger = LoggerFactory.getLogger("UtilsApi");
    private static Callback provAsyncCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String config) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], config = [" + config + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;

            String result = config;
            future.complete(result);
        }
    };

    /**
     * Provision an agent in the agency, populate configuration and wallet for this agent.
     *
     * @param  config         provisioning configuration.
     *       {
     *         protocol_type: String
     *         agency_url: String,
     *         pub agency_did: String,
     *         agency_verkey: String,
     *         wallet_name: Option(String),
     *         wallet_key: String,
     *         wallet_type: Option(String),
     *         agent_seed: Option(String),
     *         enterprise_seed: Option(String),
     *         wallet_key_derivation: Option(String),
     *         name: Option(String),
     *         logo: Option(String),
     *         path: Option(String),
     *         storage_config: Option(String),
     *         storage_credentials: Option(String),
     *         pool_config: Option(String),
     *         did_method: Option(String),
     *         communication_method: Option(String),
     *         webhook_url: Option(String),
     *         use_latest_protocols: Option(String),
     *      }
     *
     * @return                populated config that can be used for library initialization.
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static String vcxProvisionAgent(String config) {
        ParamGuard.notNullOrWhiteSpace(config, "config");
        logger.debug("vcxProvisionAgent() called with: config = [****]");
        String result = LibVcx.api.vcx_provision_agent(config);

        return result;

    }

    /**
     * Provision an agent in the agency, populate configuration and wallet for this agent.
     *
     * @param  conf           provisioning configuration.
     *       {
     *         protocol_type: String
     *         agency_url: String,
     *         pub agency_did: String,
     *         agency_verkey: String,
     *         wallet_name: Option(String),
     *         wallet_key: String,
     *         wallet_type: Option(String),
     *         agent_seed: Option(String),
     *         enterprise_seed: Option(String),
     *         wallet_key_derivation: Option(String),
     *         name: Option(String),
     *         logo: Option(String),
     *         path: Option(String),
     *         storage_config: Option(String),
     *         storage_credentials: Option(String),
     *         pool_config: Option(String),
     *         did_method: Option(String),
     *         communication_method: Option(String),
     *         webhook_url: Option(String),
     *         use_latest_protocols: Option(String),
     *      }
     *
     * @return                populated config that can be used for library initialization.
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> vcxAgentProvisionAsync(String conf) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        logger.debug("vcxAgentProvisionAsync() called with: conf = [****]");
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_agent_provision_async(
                commandHandle, conf,
                provAsyncCB);
        checkResult(result);
        return future;
    }

    /** #Params
     config: configuration

     config = {
        protocol_type: String
        agency_url: String,
        pub agency_did: String,
        agency_verkey: String,
        wallet_name: Option(String),
        wallet_key: String,
        wallet_type: Option(String),
        agent_seed: Option(String),
        enterprise_seed: Option(String),
        wallet_key_derivation: Option(String),
        name: Option(String),
        logo: Option(String),
        path: Option(String),
        storage_config: Option(String),
        storage_credentials: Option(String),
        pool_config: Option(String),
        did_method: Option(String),
        communication_method: Option(String),
        webhook_url: Option(String),
        use_latest_protocols: Option(String),
     },
     token: {
           "id": String,
           "sponsor": String, //Name of Enterprise sponsoring the provisioning
           "nonce": String,
           "timestamp": String,
           "sig": String, // Base64Encoded(sig(nonce + timestamp + id))
           "sponsor_vk": String,
         }
     **/
      public static String vcxAgentProvisionWithToken(String config, String token) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(config, "config");
        ParamGuard.notNullOrWhiteSpace(token, "token");
        logger.debug("vcxAgentProvisionWithToken() called with: config = [****], token = [***]");

        String result = LibVcx.api.vcx_provision_agent_with_token(config, token);

        return result;
    }

    /** config:
     {
      vcx_config: VcxConfig // Same config passed to agent provision
      {
            protocol_type: String
            agency_url: String,
            pub agency_did: String,
            agency_verkey: String,
            wallet_name: Option(String),
            wallet_key: String,
            wallet_type: Option(String),
            agent_seed: Option(String),
            enterprise_seed: Option(String),
            wallet_key_derivation: Option(String),
            name: Option(String),
            logo: Option(String),
            path: Option(String),
            storage_config: Option(String),
            storage_credentials: Option(String),
            pool_config: Option(String),
            did_method: Option(String),
            communication_method: Option(String),
            webhook_url: Option(String),
            use_latest_protocols: Option(String),
      }
      source_id: String // Customer Id
      com_method: {
          type: u32 // 1 means push notifcation, its the only one registered
          id: String,
          value: String,
      }
      # Example com_method -> "{"type": 1,"id":"123","value":"FCM:Value"}"
     **/
    public static CompletableFuture<Integer> vcxGetProvisionToken(String config) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(config, "config");
        logger.debug("vcxGetProvisionToken() called with: config = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_provision_token(
                commandHandle,
                config,
                vcxUpdateAgentInfoCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateAgentInfoCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    /**
     * Update information on the agent (ie, comm method and type)
     *
     * @param  config         New agent updated configuration as JSON
     *                        "{"id":"123","value":"value"}"
     *
     * @return                void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> vcxUpdateAgentInfo(String config) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(config, "config");
        logger.debug("vcxUpdateAgentInfo() called with: config = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_agent_update_info(
                commandHandle,
                config,
                vcxUpdateAgentInfoCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxGetMessagesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String messages) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], messages = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = messages;
            future.complete(result);
        }
    };

    /**
     * Retrieve messages from the agent
     *
     * @param  messageStatus  optional, comma separated - query for messages with the specified status.
     *                             Statuses:
     *                                  MS-101 - Created
     *                                  MS-102 - Sent
     *                                  MS-103 - Received
     *                                  MS-104 - Accepted
     *                                  MS-105 - Rejected
     *                                  MS-106 - Reviewed
     *                        "MS-103,MS-106"
     * @param  uids           optional, comma separated - query for messages with the specified uids
     *                        "s82g63,a2h587"
     * @param  pwdids         optional, comma separated - DID's pointing to specific connection
     *                        "did1,did2"
     *
     * @return                The list of all found messages
     *                        "[{"pairwiseDID":"did","msgs":[{"statusCode":"MS-106","payload":null,"senderDID":"","uid":"6BDkgc3z0E","type":"aries","refMsgId":null,"deliveryDetails":[],"decryptedPayload":"{"@msg":".....","@type":{"fmt":"json","name":"aries","ver":"1.0"}}"}]}]"
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> vcxGetMessages(String messageStatus, String uids, String pwdids) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(messageStatus, "messageStatus");
        logger.debug("vcxGetMessages() called with: messageStatus = [" + messageStatus + "], uids = [" + uids + "], pwdids = [****]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_messages_download(
                commandHandle,
                messageStatus,
                uids,
                pwdids,
                vcxGetMessagesCB
        );
        checkResult(result);
        return future;
    }

    /**
     * Retrieves single message from the agency by the given uid.
     *
     * @param  uid  id of the message to query.
     *
     * @return                Received message:
     *                        "{"pairwiseDID":"did","msgs":[{"statusCode":"MS-106","payload":null,"senderDID":"","uid":"6BDkgc3z0E","type":"aries","refMsgId":null,"deliveryDetails":[],"decryptedPayload":"{"@msg":".....","@type":{"fmt":"json","name":"aries","ver":"1.0"}}"}]}"
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> vcxGetMessage(String uid) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(uid, "uid");
        logger.debug("vcxGetMessage() called with: uid = [" + uid + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_download_message(
                commandHandle,
                uid,
                vcxGetMessagesCB
        );
        checkResult(result);
        return future;
    }

    /**
     * Retrieve messages from the Cloud Agent
     *
     * @param  messageStatus  optional, comma separated - query for messages with the specified status.
     *                             Statuses:
     *                                  MS-101 - Created
     *                                  MS-102 - Sent
     *                                  MS-103 - Received
     *                                  MS-104 - Accepted
     *                                  MS-105 - Rejected
     *                                  MS-106 - Reviewed
     *                        "MS-103,MS-106"
     * @param  uids           optional, comma separated - query for messages with the specified uids
     *                        "s82g63,a2h587"
     *
     * @return                The list of all found messages
     *                        "[{"pairwiseDID":"did","msgs":[{"statusCode":"MS-106","payload":null,"senderDID":"","uid":"6BDkgc3z0E","type":"aries","refMsgId":null,"deliveryDetails":[],"decryptedPayload":"{"@msg":".....","@type":{"fmt":"json","name":"aries","ver":"1.0"}}"}]}]"
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> vcxGetAgentMessages(String messageStatus, String uids) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(messageStatus, "messageStatus");
        logger.debug("vcxGetAgentMessages() called with: messageStatus = [" + messageStatus + "], uids = [" + uids + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_download_agent_messages(
                commandHandle,
                messageStatus,
                uids,
                vcxGetMessagesCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateMessagesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    /**
     * Update the status of messages from the specified connection
     *
     * @param  messageStatus  message status to set
     *                             Statuses:
     *                                  MS-101 - Created
     *                                  MS-102 - Sent
     *                                  MS-103 - Received
     *                                  MS-104 - Accepted
     *                                  MS-105 - Rejected
     *                                  MS-106 - Reviewed
     *                        "MS-103,MS-106"
     * @param  msgJson        list of messages to update
     *                        [{"pairwiseDID":"QSrw8hebcvQxiwBETmAaRs","uids":["mgrmngq"]},...]
     *
     * @return               void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> vcxUpdateMessages(String messageStatus, String msgJson) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(messageStatus, "messageStatus");
        ParamGuard.notNull(msgJson, "msgJson");
        logger.debug("vcxUpdateMessages() called with: messageStatus = [" + messageStatus + "], msgJson = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_messages_update_status(
                commandHandle,
                messageStatus,
                msgJson,
                vcxUpdateMessagesCB
        );
        checkResult(result);
        return future;
    }

    private static Callback stringCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String fees) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], fees = [" + fees + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = fees;
            future.complete(result);
        }
    };

    /**
     * Get ledger fees from the network
     *
     * @return               the fee structure for the sovrin network
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> getLedgerFees() throws VcxException {
        logger.debug("getLedgerFees() called");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_ledger_get_fees(
                commandHandle,
                stringCB
        );
        checkResult(result);
        return future;
    }

    /**
     * Retrieve author agreement and acceptance mechanisms set on the Ledger
     *
     * @return               transaction author agreement set on the ledger
     *                       "{"text":"Default agreement", "version":"1.0.0", "aml": {"label1": "description"}}"
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> getLedgerAuthorAgreement() throws VcxException {
        logger.debug("getLedgerAuthorAgreement() called");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_ledger_author_agreement(
                commandHandle,
                stringCB
        );
        checkResult(result);
        return future;
    }

    /**
     * Set some accepted agreement as active.
     * <p>
     * Either combination text/version ot hash must be passed.
     * 
     * @param  text                 Optional(string) text of transaction agreement
     * @param  version              Optional(string) version of transaction agreement
     * @param  hash                 Optional(string) hash on text and version. This parameter is required if text and version parameters are ommited.
     * @param  accMechType          mechanism how user has accepted the TAA
     * @param  timeOfAcceptance     UTC timestamp when user has accepted the TAA
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static void setActiveTxnAuthorAgreementMeta(String text, String version,
                                                         String hash, String accMechType, long timeOfAcceptance) throws VcxException {
        ParamGuard.notNull(accMechType, "accMechType");
        logger.debug("vcxProvisionAgent() called with: text = [" + text + "], version = [" + version + "]," +
                " hash = [" + hash + "], accMechType = [" + accMechType + "], timeOfAcceptance = [" + timeOfAcceptance + "]");
        int result = LibVcx.api.vcx_set_active_txn_author_agreement_meta(text, version, hash, accMechType, timeOfAcceptance);
        checkResult(result);
    }

    public static void vcxMockSetAgencyResponse(int messageIndex) {
        logger.debug("vcxMockSetAgencyResponse() called");
        LibVcx.api.vcx_set_next_agency_response(messageIndex);
    }

    public static void setPoolHandle(int handle) {
        LibVcx.api.vcx_pool_set_handle(handle);
    }

    private static Callback getReqPriceAsyncCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, long price) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], price = [" + price + "]");
            CompletableFuture<Long> future = (CompletableFuture<Long>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;

            long result = price;
            future.complete(result);
        }
    };

    /**
     * Gets minimal request price for performing an action in case the requester can perform this action.
     *
     * @param  actionJson       definition of action to get price
     *                          {
     *                              "auth_type": ledger transaction alias or associated value,
     *                              "auth_action": type of an action.,
     *                              "field": transaction field,
     *                              "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
     *                              "new_value": (Optional) new value that can be used to fill the field,
     *                          }
     * @param  requesterInfoJson  (Optional) request definition ( otherwise context info will be used).
     *                          {
     *                              "role": string - role of a user which can sign transaction.
     *                              "count": string - count of users.
     *                              "is_owner": bool - if user is an owner of transaction.
     *                          }
     *
     * @return                 price must be paid to perform the requested action
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Long> vcxGetRequestPrice(String actionJson, String requesterInfoJson) throws VcxException {
        ParamGuard.notNull(actionJson, "actionJson");
        logger.debug("vcxGetRequestPrice() called with: actionJson = [" + actionJson + "], requesterInfoJson = [" + requesterInfoJson + "]");
        CompletableFuture<Long> future = new CompletableFuture<Long>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_request_price(
                commandHandle, actionJson, requesterInfoJson,
                getReqPriceAsyncCB);
        checkResult(result);
        return future;
    }

    private static Callback vcxEndorseTransactionCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    /**
     * Endorse transaction to the ledger preserving an original author
     *
     * @param  transactionJson  transaction to endorse
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> vcxEndorseTransaction(String transactionJson) throws VcxException {
        ParamGuard.notNull(transactionJson, "transactionJson");
        logger.debug("vcxEndorseTransaction() called with: transactionJson = [" + transactionJson + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_endorse_transaction(
                commandHandle, transactionJson,
                vcxEndorseTransactionCb);
        checkResult(result);
        return future;
    }
}
