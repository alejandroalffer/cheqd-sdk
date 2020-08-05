package com.evernym.sdk.vcx.indy;

import com.evernym.sdk.vcx.VcxJava;
import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;

import com.sun.jna.*;

import java9.util.concurrent.CompletableFuture;

public class IndyApi extends VcxJava.API {

  	/**
	 * Callback used when buildRequest completes
	 */
	private static Callback buildRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};


	/**
	 * Builds a TXN_AUTHR_AGRMT request. Request to add a new version of Transaction Author Agreement to the ledger.
	 *
	 * EXPERIMENTAL
	 *
	 * @param submitterDid DID of the request sender.
	 * @param text -  a content of the TTA.
	 * @param version -  a version of the TTA (unique UTF-8 string).
	 *
	 * @return A future resolving to a request result as json.
	 * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> addTxnAuthorAgreement(
			String submitterDid,
			String text,
			String version) throws VcxException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNull(text, "text");
		ParamGuard.notNull(version, "version");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.indy_build_txn_author_agreement_request(
				commandHandle,
				submitterDid,
				text,
				version,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}


	/**
	 * Builds a GET_TXN_AUTHR_AGRMT request. Request to get a specific Transaction Author Agreement from the ledger.
	 *
	 * EXPERIMENTAL
	 *
	 * @param submitterDid (Optional) DID of the request sender.
	 * @param data -  (Optional) specifies a condition for getting specific TAA.
	 * Contains 3 mutually exclusive optional fields:
	 * {
	 *     hash: Optional[str] - hash of requested TAA,
	 *     version: Optional[str] - version of requested TAA.
	 *     timestamp: Optional[u64] - ledger will return TAA valid at requested timestamp.
	 * }
	 * Null data or empty JSON are acceptable here. In this case, ledger will return the latest version of TAA.
	 *
	 * @return A future resolving to a request result as json.
	 * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> getTxnAuthorAgreement(
			String submitterDid,
			String data) throws VcxException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.indy_build_get_txn_author_agreement_request(
				commandHandle,
				submitterDid,
				data,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}


    /**
     * Builds a SET_TXN_AUTHR_AGRMT_AML request. Request to add a new list of acceptance mechanisms for transaction author agreement.
     * Acceptance Mechanism is a description of the ways how the user may accept a transaction author agreement.
     *
     * EXPERIMENTAL
     *
     * @param submitterDid DID of the request sender.
     * @param aml - a set of new acceptance mechanisms:
     * <pre>
     * {@code
     * {
     *     "<acceptance mechanism label 1>": { acceptance mechanism description 1},
     *     "<acceptance mechanism label 2>": { acceptance mechanism description 2},
     *     ...
     * }
     * }
     * </pre>
     *
     * @param version - a version of new acceptance mechanisms. (Note: unique on the Ledger).
     * @param amlContext - (Optional) common context information about acceptance mechanisms (may be a URL to external resource).
     *
     * @return A future resolving to a request result as json.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<String> addAcceptanceMechanisms(
        String submitterDid,
        String aml,
        String version,
        String amlContext) throws VcxException {

      ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
      ParamGuard.notNull(aml, "aml");
      ParamGuard.notNull(version, "version");

      CompletableFuture<String> future = new CompletableFuture<String>();
      int commandHandle = addFuture(future);

      int result = LibVcx.api.indy_build_acceptance_mechanisms_request(
          commandHandle,
          submitterDid,
          aml,
          version,
          amlContext,
          buildRequestCb);

      checkResult(future, result);

      return future;
    }


    /**
     * Builds a GET_TXN_AUTHR_AGRMT_AML request. Request to get a list of  acceptance mechanisms from the ledger
     * valid for specified time or the latest one.
     *
     * EXPERIMENTAL
     *
     * @param submitterDid (Optional) DID of the request sender.
     * @param timestamp - time to get an active acceptance mechanisms. Pass -1 to get the latest one.
     * @param version - (Optional) version of acceptance mechanisms.
     *
     * NOTE: timestamp and version cannot be specified together.
     *
     * @return A future resolving to a request result as json.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<String> getAcceptanceMechanisms(
        String submitterDid,
        Long timestamp,
        String version) throws VcxException {

      CompletableFuture<String> future = new CompletableFuture<String>();
      int commandHandle = addFuture(future);

      int result = LibVcx.api.indy_build_get_acceptance_mechanisms_request(
          commandHandle,
          submitterDid,
          timestamp,
          version,
          buildRequestCb);

      checkResult(future, result);

      return future;
    }


    /**
     * Append transaction author agreement acceptance data to a request.
     * This function should be called before signing and sending a request
     * if there is any transaction author agreement set on the Ledger.
     *
     * EXPERIMENTAL
     *
     * This function may calculate digest by itself or consume it as a parameter.
     * If all text, version and taaDigest parameters are specified, a check integrity of them will be done.
     *
     * @param requestJson original request data json.
     * @param text - (Optional) raw data about TAA from ledger.
     * @param version - (Optional) raw version about TAA from ledger.
     *     `text` and `version` parameters should be passed together.
     *     `text` and `version` parameters are required if taaDigest parameter is omitted.
     * @param taaDigest - (Optional) digest on text and version. This parameter is required if text and version parameters are omitted.
     * @param mechanism - mechanism how user has accepted the TAA
     * @param time - UTC timestamp when user has accepted the TAA. Note that the time portion will be discarded to avoid a privacy risk.
     *
     * @return A future resolving to an updated request result as json.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<String> appendTxnAuthorAgreement(
        String requestJson,
        String text,
        String version,
        String taaDigest,
        String mechanism,
        long time) throws VcxException {

      ParamGuard.notNull(requestJson, "requestJson");
      ParamGuard.notNull(mechanism, "mechanism");

      CompletableFuture<String> future = new CompletableFuture<String>();
      int commandHandle = addFuture(future);

      int result = LibVcx.api.indy_append_txn_author_agreement_acceptance_to_request(
          commandHandle,
          requestJson,
          text,
          version,
          taaDigest,
          mechanism,
          time,
          buildRequestCb);

      checkResult(future, result);

      return future;
    }


    /**
     * Callback used when cryptoBoxSeal encrypt completes.
     */
    private static Callback anonCryptCb = new Callback() {

      @SuppressWarnings({"unused", "unchecked"})
      public void callback(int xcommand_handle, int err, Pointer encrypted_msg_raw, int encrypted_msg_len) {

        CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
        if (! checkResult(future, err)) return;

        byte[] encryptedMsg = new byte[encrypted_msg_len];
        encrypted_msg_raw.read(0, encryptedMsg, 0, encrypted_msg_len);

        future.complete(encryptedMsg);
      }
    };

    /**
	 * Encrypts a message by anonymous-encryption scheme.
	 *
	 * Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
	 * Only the Recipient can decrypt these messages, using its private key.
	 * While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
	 *
	 * Note to use DID keys with this function you can call keyForDid to get key id (verkey)
	 * for specific DID.
	 *
	 * @param recipientVk verkey of message recipient
	 * @param message a message to be signed
	 * @return A future that resolves to an encrypted message as an array of bytes.
	 * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> anonCrypt(
        String recipientVk,
        byte[] message) throws VcxException {

        ParamGuard.notNullOrWhiteSpace(recipientVk, "theirVk");
        ParamGuard.notNull(message, "message");

        CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.indy_crypto_anon_crypt(
                commandHandle,
                recipientVk,
                message,
                message.length,
                anonCryptCb);

        checkResult(future, result);

        return future;
    }


    /**
     * Callback used when cryptoBoxSealOpen completes.
     */
    private static Callback anonDecryptCb = new Callback() {

      @SuppressWarnings({"unused", "unchecked"})
      public void callback(int xcommand_handle, int err, Pointer decrypted_msg_raw, int decrypted_msg_len) {

        CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
        if (! checkResult(future, err)) return;

        byte[] result = new byte[decrypted_msg_len];
        decrypted_msg_raw.read(0, result, 0, decrypted_msg_len);
        future.complete(result);
      }
    };


    /**
     * Decrypts a message by anonymous-encryption scheme.
     *
     * Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
     * Only the Recipient can decrypt these messages, using its private key.
     * While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
     *
     * Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
     * for specific DID.
     *
     * @param walletHandle       The walletHandle.
     * @param recipientVk  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
     * @param encryptedMsg encrypted message
     * @return A future that resolves to a decrypted message as an array of bytes.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<byte[]> anonDecrypt(
            int walletHandle,
            String recipientVk,
            byte[] encryptedMsg) throws VcxException {

        //ParamGuard.notNull(wallet, "wallet");
        ParamGuard.notNullOrWhiteSpace(recipientVk, "myVk");
        ParamGuard.notNull(encryptedMsg, "encryptedMsg");

        CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
        int commandHandle = addFuture(future);

        //int walletHandle = wallet.getWalletHandle();

        int result = LibVcx.api.indy_crypto_anon_decrypt(
                commandHandle,
                walletHandle,
                recipientVk,
                encryptedMsg,
                encryptedMsg.length,
                anonDecryptCb);

        checkResult(future, result);

        return future;
    }

}