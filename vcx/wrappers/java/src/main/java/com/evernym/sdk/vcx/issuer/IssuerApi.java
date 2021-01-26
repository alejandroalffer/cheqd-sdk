package com.evernym.sdk.vcx.issuer;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.util.*;
import java9.util.concurrent.CompletableFuture;

public class IssuerApi extends VcxJava.API {

    private static final Logger logger = LoggerFactory.getLogger("IssuerApi");
    private static final Callback issuerCreateCredentialCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int credentialHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], credentialHandle = [" + credentialHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = credentialHandle;
            future.complete(result);
        }
    };

    /**
     * Create a IssuerCredential object that provides a credential for an enterprise's user
     *
     * @param  sourceId             Enterprise's personal identification for the credential, should be unique.
     * @param  credentialDefHandle  handle pointing to CredentialDefinition to use for issuance. 
     *                              It must be already stored in the wallet and written to the ledger.
     * @param  issuerId             DID corresponding to entity issuing a credential. Needs to have Trust Anchor permissions on ledger
     * @param  credentialData       List of attributes offered credential will contain.
     *                              "{"state":"UT"}"
     * @param  credentialName       Human-readable name of the credential - ex. Drivers Licence
     * @param  price                price user have to pay to receive credential.
     *
     * @return                      handle that should be used to perform actions with the IssuerCredential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCreateCredential(String sourceId,
                                                                    int credentialDefHandle,
                                                                    String issuerId,
                                                                    String credentialData,
                                                                    String credentialName,
                                                                    String price) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(credentialData, "credentialData");
        ParamGuard.notNullOrWhiteSpace(credentialName, "credentialName");

        logger.debug("issuerCreateCredential() called with: sourceId = [" + sourceId + "], credentialDefHandle = [" + credentialDefHandle + "], issuerId = [****, credentialData = [****], credentialName = [****], price = [****]");
        //TODO: Check for more mandatory params in vcx to add in PamaGuard
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_create_credential(
                issue,
                sourceId,
                credentialDefHandle,
                issuerId,
                credentialData,
                credentialName,
                price,
                issuerCreateCredentialCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerSendCredentialOfferCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(null);
        }
    };

    /**
     * Send a Credential Offer to user showing what will be included in the actual credential.
     *
     * @param  credentialHandle  handle pointing to IssuerCredential object.
     * @param  connectionHandle  handle pointing to Connection object to use for message sending. 
     *                           
     * @return                   void
     *
     * @throws VcxException      If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Void> issuerSendCredentialOffer(int credentialHandle,
                                                                       int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("issuerSendcredentialOffer() called with: credentialOffer = [" + credentialHandle + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Void> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential_offer(
                issue,
                credentialHandle,
                connectionHandle,
                issuerSendCredentialOfferCB
        );
        checkResult(result);
        return future;
    }

    /**
     * Gets the Credential Offer message that can be sent to the specified connection
     *
     * @param  credentialHandle  handle pointing to IssuerCredential object.
     *
     * @return                   Credential Offer message as JSON string
     *
     * @throws VcxException      If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> issuerGetCredentialOfferMsg(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerSendCredentialOffer() called with: credentialHandle = [****]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_get_credential_offer_msg(
                issue,
                credentialHandle,
                issuerCredentialStringCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err,int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state);
        }
    };

    /**
     * Query the agency for the received messages.
     * Checks for any messages changing state in the IssuerCredential object and updates the state attribute.
     *
     * @param  credentialHandle  handle pointing to IssuerCredential object.
     *
     * @return                   the most current state of IssuerCredential object.
     *
     * @throws VcxException      If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCredentialUpdateState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredentialUpdateState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_credential_update_state(issue, credentialHandle, issuerCredentialUpdateStateCB);
        checkResult(result);
        return future;
    }

    /**
     * Update the state of the IssuerCredential object based on the given message.
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     * @param  message              message to process for any IssuerCredential state transitions.
     *
     * @return                      the most current state of the IssuerCredential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCredentialUpdateStateWithMessage(int credentialHandle, String message) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(message, "message");

        logger.debug("issuerCredentialUpdateStateWithMessage() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_update_state_with_message(issue, credentialHandle, message, issuerCredentialUpdateStateCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state);
        }
    };

    /**
     * Get the current state of the IssuerCredential object
     * Credential states:
     *         1 - Initialized
     *         2 - Credential Offer Sent
     *         3 - Credential Request Received
     *         4 - Credential Issued
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     *
     * @return                      the most current state of the IssuerCredential object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCredentialGetState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredentialGetState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_get_state(issue, credentialHandle, issuerCredentialGetStateCB);
        checkResult(result);
        return future;
    }
    private static Callback issuerSendCredentialCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(null);
        }
    };

    /**
     * Sends the Credential message to the end user (holder).
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     * @param  connectionHandle     handle pointing to a Connection object to use for message sending.
     *
     * @return                      the most current state of the IssuerCredential object.
     *
     * @throws VcxException         void
     */
    public static CompletableFuture<Void> issuerSendCredential(int credentialHandle,
                                                                 int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("issuerSendCredential() called with: credentialHandle = [" + credentialHandle + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Void> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential(
                issue,
                credentialHandle,
                connectionHandle,
                issuerSendCredentialCB);

        checkResult(result);
        return future;
    }

    /**
     * Gets the Credential message that can be sent to the user.
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     * @param  myPwDid              Pairwise key used for Connection set up (use ConnectionApi.connectionGetPwDid to get).
     *
     * @return                      Credential message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> issuerGetCredentialMsg(int credentialHandle,
                                                                   String myPwDid) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerGetCredentialMsg() called with: credentialHandle = [****]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_get_credential_msg(
                issue,
                credentialHandle,
                myPwDid,
                issuerCredentialStringCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialStringCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String stringData) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], string = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = stringData;
            future.complete(result);
        }
    };

    /**
     * Get JSON string representation of IssuerCredential object.
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     *
     * @return                      IssuerCredential object as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> issuerCredentialSerialize(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredentialSerialize() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_serialize(
                issue,
                credentialHandle,
                issuerCredentialStringCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int handle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], handle = [" + handle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = handle;
            future.complete(result);
        }
    };

    /**
     * Takes a json string representing a IssuerCredential object and recreates an object matching the JSON.
     *
     * @param  serializedData  JSON string representing a IssuerCredential object.
     *
     * @return                 handle that should be used to perform actions with the IssuerCredential object.
     *
     * @throws VcxException    If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCredentialDeserialize(String serializedData) throws VcxException {
        ParamGuard.notNull(serializedData, "serializedData");
        logger.debug("issuerCredentialDeserialize() called with: serializedData = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_deserialize(
                issue,
                serializedData,
                issuerCredentialDeserializeCB
        );
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerTerminateCredential(
            int credentialHandle,
            int state,
            String msg
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(state, "state");
        ParamGuard.notNullOrWhiteSpace(msg, "msg");
        logger.debug("issuerTerminateCredential() called with: credentialHandle = [" + credentialHandle + "], state = [" + state + "], msg = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_terminate_credential(
                issue,
                credentialHandle,
                state,
                msg);
        checkResult(result);

        return future;

    }

    /**
     * Releases the IssuerCredential object by de-allocating memory
     *
     * @param  credentialHandle     handle pointing to a IssuerCredential object.
     *
     * @return                      void
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static int issuerCredentialRelease(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredentialRelease() called with: credentialHandle = [" + credentialHandle + "]");

        int result = LibVcx.api.vcx_issuer_credential_release(credentialHandle);
        checkResult(result);

        return result;
    }

    /**
     * Gets the Credential Request message that can be sent to the user.
     *
     * WARN: Outdated function that MUST NOT be used.
     *
     * @param  credentialHandle     -
     * @param  credentialRequest    -
     *
     * @return                      -
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> issuerCredentialRequest(
            int credentialHandle,
            String credentialRequest) throws VcxException {

        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(credentialRequest, "credentialRequest");
        logger.debug("issuercredentialRequest() called with: credentialHandle = [" + credentialHandle + "], credentialRequest = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_get_credential_request(
                credentialHandle,
                credentialRequest);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> issuerAcceptRequest(
            int credentialHandle) throws VcxException {

        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerAcceptRequest() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_accept_credential(
                credentialHandle);
        checkResult(result);

        return future;
    }

    /**
     * Get Problem Report message for object in Failed or Rejected state.
     *
     * @param  credentialHandle handle pointing to Issuer state object.
     *
     * @return                  Problem Report as JSON string or null
     *
     * @throws VcxException     If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> issuerGetProblemReport(
            int credentialHandle
    ) throws VcxException {

        logger.debug("issuerGetProblemReport() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_get_problem_report(commandHandle, credentialHandle, issuerCredentialStringCB);
        checkResult(result);

        return future;
    }
}
