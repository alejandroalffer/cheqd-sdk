package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class ProofApi extends VcxJava.API {
    private ProofApi(){}

    private static final Logger logger = LoggerFactory.getLogger("ProofApi");
    
    private static Callback vcxProofCreateCB = new Callback() {
        public void callback(int commandHandle, int err, int proofHandle){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], proofHandle = [" + proofHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            Integer result = proofHandle;
            future.complete(result);
        }
    };

    /**
     * Create a new Proof object that requests a proof for an enterprise
     *
     * @param  sourceId             Enterprise's personal identification for the proof, should be unique.
     * @param  requestedAttrs       Describes the list of requested attribute
     *     [{
     *         "name": Optional(string), // attribute name, (case insensitive and ignore spaces)
     *         "names": Optional([string, string]), // attribute names, (case insensitive and ignore spaces)
     *                                              // NOTE: should either be "name" or "names", not both and not none of them.
     *                                              // Use "names" to specify several attributes that have to match a single credential.
     *         "restrictions":  Optional(wql query) - set of restrictions applying to requested credentials. (see below)
     *         "non_revoked": {
     *             "from": Optional(u64) Requested time represented as a total number of seconds from Unix Epoch, Optional
     *             "to": Optional(u64)
     *                 //Requested time represented as a total number of seconds from Unix Epoch, Optional
     *         }
     *     }]                               
     * @param  requestedPredicates  predicate specifications prover must provide claim for.
     *     <pre>
     *     {@code
     *     [
     *        { // set of requested predicates
     *           "name": attribute name, (case insensitive and ignore spaces)
     *           "p_type": predicate type (Currently ">=" only)
     *           "p_value": int predicate value
     *           "restrictions":  Optional(wql query) -  set of restrictions applying to requested credentials. (see below)
     *           "non_revoked": Optional({
     *               "from": Optional(u64) Requested time represented as a total number of seconds from Unix Epoch, Optional
     *               "to": Optional(u64) Requested time represented as a total number of seconds from Unix Epoch, Optional
     *           })
     *       }
     *    ]
     *    }
     *    </pre>
     *                                            
     * @param  revocationInterval  Optional timestamps to request revocation proof
     * @param  name                label for proof request.
     *
     * @return                      handle that should be used to perform actions with the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofCreate(
            String sourceId,
            String requestedAttrs,
            String requestedPredicates,
            String revocationInterval,
            String name
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(requestedAttrs, "requestedAttrs");
        ParamGuard.notNull(requestedPredicates, "requestedPredicates");
        ParamGuard.notNull(revocationInterval, "revocationInterval");
        ParamGuard.notNull(name, "name");
        logger.debug("proofCreate() called with: sourceId = [" + sourceId + "], requestedAttrs = [****], requestedPredicates = [****], revocationInterval = [****], name = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        if (requestedPredicates.isEmpty()) requestedPredicates = "[]";
        int result = LibVcx.api.vcx_proof_create(commandHandle, sourceId, requestedAttrs, requestedPredicates, revocationInterval, name, vcxProofCreateCB);
        checkResult(result);

        return future;
    }

    /**
     * Create a new Proof object based on the given Presentation Proposal message
     *
     * @param  sourceId             Enterprise's personal identification for the proof, should be unique.
     * @param  presentationProposal Message sent by the Prover to the verifier to initiate a proof presentation process:
     *         {
     *             "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/propose-presentation",
     *             "@id": "<uuid-propose-presentation>",
     *             "comment": "some comment",
     *             "presentation_proposal": {
     *                 "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/presentation-preview",
     *                 "attributes": [
     *                     {
     *                         "name": "<attribute_name>", - name of the attribute.
     *                         "cred_def_id": "<cred_def_id>", - maps to the credential definition identifier of the credential with the current attribute
     *                         "mime-type": Optional"<type>", - optional type of value. if mime-type is missing (null), then value is a string.
     *                         "value": "<value>", - value of the attribute to reveal in presentation
     *                     },
     *                     // more attributes
     *                   ],
     *                  "predicates": [
     *                     {
     *                         "name": "<attribute_name>", - name of the attribute.
     *                         "cred_def_id": "<cred_def_id>", - maps to the credential definition identifier of the credential with the current attribute
     *                         "predicate": "<predicate>", - predicate operator: "<", "<=", ">=", ">"
     *                         "threshold": <threshold> - threshold value for the predicate.
     *                     },
     *                     // more predicates
     *                 ]
     *             }
     *         }
     *                                            
     * @param  name                 label for proof request.
     *
     * @return                      handle that should be used to perform actions with the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofCreateWithProposal(
            String sourceId,
            String presentationProposal,
            String name
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(presentationProposal, "presentationProposal");
        ParamGuard.notNull(name, "name");
        logger.debug("proofCreateWithProposal() called with: sourceId = [" + sourceId + "], presentationProposal = [" + presentationProposal + "], name = [" + name + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_proof_create_with_proposal(commandHandle, sourceId, presentationProposal, name, vcxProofCreateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofSendRequestCB = new Callback() {
        public void callback(int commandHandle, int err){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            future.complete(null);
        }
    };

    /**
     * Sends a Proof Request message to pairwise connection.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     * @param  connectionHandle     handle pointing to a Connection object to use for sending message.
     *
     * @return                      void
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Void> proofSendRequest(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("proofSendRequest() called with: proofHandle = [" + proofHandle + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_send_request(commandHandle, proofHandle, connectionHandle, vcxProofSendRequestCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofGetRequestMsgCB = new Callback() {
        public void callback(int commandHandle, int err, String msg){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], msg = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            Integer result = commandHandle;
            future.complete(msg);
        }
    };

    /**
     * Get Proof Request message that can be sent to the pairwise connection.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      Proof Request message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> proofGetRequestMsg(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("proofGetRequestMsg() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_get_request_msg(commandHandle, proofHandle, vcxProofGetRequestMsgCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxGetProofCB = new Callback() {
        public void callback(int commandHandle, int err, int proofState, String responseData){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], proofState = [" + proofState + "], responseData = [****]");
            CompletableFuture<GetProofResult> future = (CompletableFuture<GetProofResult>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            GetProofResult result = new GetProofResult(proofState,responseData);
            future.complete(result);
        }
    };

    /**
     * Get Proof message that can be sent to the specified connection.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     * @param  connectionHandle     handle pointing to a Connection object.
     *
     * @return                      Proof message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<GetProofResult> getProof(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("getProof() called with: proofHandle = [" + proofHandle + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<GetProofResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_proof(commandHandle, proofHandle, connectionHandle, vcxGetProofCB);
        checkResult(result);

        return future;
    }

    /**
     * Get Proof message that can be sent to the specified connection.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      Proof message as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<GetProofResult> getProofMsg(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("getProof() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<GetProofResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_proof_msg(commandHandle, proofHandle, vcxGetProofCB);
        checkResult(result);

        return future;
    }


    // vcx_proof_accepted
    public static CompletableFuture<Integer> proofAccepted(
            int proofHandle,
            String responseData
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(responseData, "responseData");
        logger.debug("proofAccepted() called with: proofHandle = [" + proofHandle + "], responseData = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_proof_accepted(proofHandle, responseData);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofUpdateStateCB = new Callback() {
        public void callback(int commandHandle, int err, int state){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    /**
     * Query the agency for the received messages.
     * Checks for any messages changing state in the Proof object and updates the state attribute.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      the most current state of the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofUpdateState(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("proofUpdateState() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_update_state(commandHandle, proofHandle, vcxProofUpdateStateCB);
        checkResult(result);

        return future;
    }

    /**
     * Update the state of the Proof object based on the given message.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     * @param  message              message to process for any Proof state transitions.
     *
     * @return                      the most current state of the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofUpdateStateWithMessage(
            int proofHandle,
            String message
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(message, "message");

        logger.debug("proofUpdateStateWithMessage() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_update_state_with_message(commandHandle, proofHandle, message, vcxProofUpdateStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofGetStateCB = new Callback() {
        public void callback(int commandHandle, int err, int state){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    /**
     * Get the current state of the Proof object
     * Proof states:
     *     1 - Initialized
     *     2 - Proof Request Sent
     *     3 - Proof Received
     *     4 - Proof Accepted
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      the most current state of the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofGetState(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("proofGetState() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_get_state(commandHandle, proofHandle, vcxProofGetStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofSerializeCB = new Callback() {
        public void callback(int commandHandle, int err, String proofState){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], proofState = [" + proofState + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            future.complete(proofState);
        }
    };

    /**
     * Get JSON string representation of Proof object.
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      Proof object as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<String> proofSerialize(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("proofSerialize() called with: proofHandle = [" + proofHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_serialize(commandHandle, proofHandle, vcxProofSerializeCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofDeserializeCB = new Callback() {
        public void callback(int commandHandle, int err, int proofHandle){
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], proofHandle = [" + proofHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if(!checkCallback(future,err)) return;
            Integer result = proofHandle;
            future.complete(result);
        }
    };

    /**
     * Takes a json string representing a Proof object and recreates an object matching the JSON.
     *
     * @param  serializedProof      JSON string representing a Proof object.
     *
     * @return                      handle that should be used to perform actions with the Proof object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> proofDeserialize(
            String serializedProof
    ) throws VcxException {
        ParamGuard.notNull(serializedProof, "serializedProof");
        logger.debug("proofDeserialize() called with: serializedProof = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_deserialize(commandHandle, serializedProof, vcxProofDeserializeCB);
        checkResult(result);

        return future;
    }

    /**
     * Releases the Proof object by de-allocating memory
     *
     * @param  proofHandle          handle pointing to a Proof object.
     *
     * @return                      void
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static int proofRelease(int proofHandle) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        logger.debug("proofRelease() called with: proofHandle = [" + proofHandle + "]");

        int result = LibVcx.api.vcx_proof_release(proofHandle);
        checkResult(result);

        return result;
    }

}
