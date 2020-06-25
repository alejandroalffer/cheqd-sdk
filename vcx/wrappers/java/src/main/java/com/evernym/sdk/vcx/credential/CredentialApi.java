package com.evernym.sdk.vcx.credential;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class CredentialApi extends VcxJava.API {

    private static final Logger logger = LoggerFactory.getLogger("CredentialApi");
    private CredentialApi() {
    }

    private static Callback vcxCredentialCreateWithMsgidCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credentialHandle, String offer) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], credentialHandle = [" + credentialHandle + "], offer = [****]");
            CompletableFuture<GetCredentialCreateMsgidResult> future = (CompletableFuture<GetCredentialCreateMsgidResult>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            GetCredentialCreateMsgidResult result = new GetCredentialCreateMsgidResult(credentialHandle, offer);
            future.complete(result);
        }
    };

    /**
     * Create a Credential object based off of a known message id (containing Credential Offer) for a given connection.
     *
     * @param  sourceId             Institution's personal identification for the credential. It'll be used as a label.
     * @param  connectionHandle     handle pointing to a Connection object to query for credential offer message.
     * @param  msgId                id of the message on Agency that contains the credential offer.
     *
     * @return                      GetCredentialCreateMsgidResult object that contains
     *                               - handle that should be used to perform actions with the Credential object.
     *                               - Credential Offer message as JSON string
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<GetCredentialCreateMsgidResult> credentialCreateWithMsgid(
            String sourceId,
            int connectionHandle,
            String msgId
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(msgId, "msgId");
        logger.debug("credentialCreateWithMsgid() called with: sourceId = [" + sourceId + "], connectionHandle = [" + connectionHandle + "], msgId = [" + msgId + "]");
        CompletableFuture<GetCredentialCreateMsgidResult> future = new CompletableFuture<GetCredentialCreateMsgidResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_create_with_msgid(
                commandHandle,
                sourceId,
                connectionHandle,
                msgId,
                vcxCredentialCreateWithMsgidCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialSendRequestCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            // returning empty string from here because we don't want to complete future with null
            future.complete("");
        }
    };

    /**
     * Approves the Credential Offer and submits a Credential Request.
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     * @param  connectionHandle     handle pointing to a Connection object.
     * @param  paymentHandle        deprecated parameter (use 0).
     *
     * @return                      void
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> credentialSendRequest(
            int credentialHandle,
            int connectionHandle,
            int paymentHandle
    ) throws VcxException {
        logger.debug("credentialSendRequest() called with: credentialHandle = [" + credentialHandle + "], connectionHandle = [" + connectionHandle + "], paymentHandle = [" + paymentHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_send_request(
                commandHandle,
                credentialHandle,
                connectionHandle,
                paymentHandle,
                vcxCredentialSendRequestCB);
        checkResult(result);

        return future;

    }

    /**
     * Approves the Credential Offer and gets the Credential Request message that can be sent to the specified connection
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     * @param  myPwDid              pairwise DID used for Connection.
     * @param  theirPwDid           pairwise DID of the remote side used for Connection.
     * @param  paymentHandle        deprecated parameter (use 0).
     *
     * @return                      Credential Request message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> credentialGetRequestMsg(
            int credentialHandle,
            String myPwDid,
            String theirPwDid,
            int paymentHandle
    ) throws VcxException {
        logger.debug("credentialGetRequestMsg() called with: credentialHandle = [" + credentialHandle + "], myPwDid = [" + myPwDid + "], theirPwDid = [" + theirPwDid + "], paymentHandle = [" + paymentHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_get_request_msg(
                commandHandle,
                credentialHandle,
                myPwDid,
                theirPwDid,
                paymentHandle,
                vcxCredentialStringCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialStringCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String stringData) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], string = [" + stringData + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(stringData);
        }
    };

    /**
     * Get JSON string representation of Credential object.
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     *
     * @return                      Credential object as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> credentialSerialize(
            int credentialHandle
    ) throws VcxException {
        logger.debug("credentialSerialize() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_serialize(commandHandle,
                credentialHandle,
                vcxCredentialStringCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credentialHandle) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], credentialHandle = [" + credentialHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = credentialHandle;
            future.complete(result);
        }
    };

    /**
     * Takes a json string representing a Credential object and recreates an object matching the JSON.
     *
     * @param  serializedCredential JSON string representing a Credential object.
     *
     * @return                      handle that should be used to perform actions with the Credential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> credentialDeserialize(
            String serializedCredential
    ) throws VcxException {
        ParamGuard.notNull(serializedCredential, "serializedCredential");
        logger.debug("credentialDeserialize() called with: serializedCredential = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_deserialize(commandHandle,
                serializedCredential,
                vcxCredentialDeserializeCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxGetCredentialCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String credential) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], credential = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(credential);
        }
    };

    /**
     * Retrieve information about a stored credential in user's wallet, including credential id and the credential itself.
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     *
     * @return                      Credential message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> getCredential(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("getCredential() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_credential(commandHandle, credentialHandle, vcxGetCredentialCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int state) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    /**
     * Query the agency for the received messages.
     * Checks for any messages changing state in the Credential object and updates the state attribute.
     * If it detects a credential it will store the credential in the wallet.
     * 
     * @param  credentialHandle     handle pointing to a Credential object.
     *
     * @return                      the most current state of the Credential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> credentialUpdateState(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("credentialUpdateState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_update_state(commandHandle, credentialHandle, vcxCredentialUpdateStateCB);
        checkResult(result);

        return future;
    }

    /**
     * Update the state of the Credential object based on the given message.
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     * @param  message              message to process for any Credential state transitions.
     *
     * @return                      the most current state of the Credential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> credentialUpdateStateWithMessage(
            int credentialHandle,
            String message
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("credentialUpdateState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_update_state_with_message(commandHandle, credentialHandle, message, vcxCredentialUpdateStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int state) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    /**
     * Get the current state of the Credential object
     * Credential states:
     *     2 - Credential Request Sent
     *     3 - Credential Offer Received
     *     4 - Credential Accepted
     * 
     * @param  credentialHandle     handle pointing to a Credential object.
     *
     * @return                      the most current state of the Credential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> credentialGetState(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("credentialGetState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_get_state(commandHandle, credentialHandle, vcxCredentialGetStateCB);
        checkResult(result);

        return future;
    }

    /**
     * Releases the Credential object by de-allocating memory
     *
     * @param  credentialHandle     handle pointing to a Credential object.
     *
     * @return                      void
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static int credentialRelease(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("credentialRelease() called with: credentialHandle = [" + credentialHandle + "]");

        int result = LibVcx.api.vcx_credential_release(credentialHandle);
        checkResult(result);

        return result;
    }

    private static Callback vcxCredentialGetOffersCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String credential_offers) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], credential_offers = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(credential_offers);
        }
    };

    /**
     * Queries agency for Credential Offer messages from the given connection.
     *
     * @param  connectionHandle     handle pointing to Connection object to query for credential offers.
     *
     * @return                      List of received Credential Offers as JSON string.
     *                              "[[{"msg_type": "CREDENTIAL_OFFER","version": "0.1","to_did": "...","from_did":"...","credential": {"account_num": ["...."],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "...","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}]]"
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> credentialGetOffers(
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("credentialGetOffers() called with: connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_get_offers(commandHandle, connectionHandle, vcxCredentialGetOffersCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialCreateWithOfferCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credential_handle) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], credential_handle = [" + credential_handle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = credential_handle;
            future.complete(result);
        }
    };

    /**
     * Create a Credential object that requests and receives a credential for an institution
     *
     * @param  sourceId         Institution's personal identification for the credential, should be unique.
     * @param  credentialOffer  Received Credential Offer message.
     *                          The format of Credential Offer depends on communication method:
     *                              proprietary:
     *                                  "[{"msg_type": "CREDENTIAL_OFFER","version": "0.1","to_did": "...","from_did":"...","credential": {"account_num": ["...."],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "...","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}]"
     *                              aries:
     *                                  "{"@type":"did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/offer-credential", "@id":"<uuid-of-offer-message>", "comment":"somecomment", "credential_preview":<json-ldobject>, "offers~attach":[{"@id":"libindy-cred-offer-0", "mime-type":"application/json", "data":{"base64":"<bytesforbase64>"}}]}"
     *
     * @return                      handle that should be used to perform actions with the Credential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> credentialCreateWithOffer(
            String sourceId,
            String credentialOffer
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(credentialOffer, "credentialOffer");
        logger.debug("credentialCreateWithOffer() called with: sourceId = [" + sourceId + "], credentialOffer = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_create_with_offer(commandHandle, sourceId, credentialOffer, vcxCredentialCreateWithOfferCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxAcceptCredentialOfferCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credentialHandle, String credentialSerialized) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], " +
                    "credentialHandle = [" + credentialHandle + "], offer = [****]");
            CompletableFuture<CredentialAcceptOfferResult> future = (CompletableFuture<CredentialAcceptOfferResult>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            CredentialAcceptOfferResult result = new CredentialAcceptOfferResult(credentialHandle, credentialSerialized);
            future.complete(result);
        }
    };

    public static CompletableFuture<CredentialAcceptOfferResult> acceptCredentialOffer(
            String sourceId,
            String credentialOffer,
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNull(credentialOffer, "credentialOffer");
        ParamGuard.notNull(connectionHandle, "connectionHandle");

        logger.debug("acceptCredentialOffer() called with: sourceId = [" + sourceId + "], credentialOffer = [" + credentialOffer + "], " +
                "connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<CredentialAcceptOfferResult> future = new CompletableFuture<CredentialAcceptOfferResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_accept_credential_offer(
                commandHandle,
                sourceId,
                credentialOffer,
                connectionHandle,
                vcxAcceptCredentialOfferCB);
        checkResult(result);

        return future;
    }
}
