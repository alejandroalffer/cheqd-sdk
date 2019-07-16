package com.evernym.sdk.vcx.indy;

import com.evernym.sdk.vcx.VcxJava;
import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;

import com.sun.jna.*;

import java9.util.concurrent.CompletableFuture;

public class IndyApi extends VcxJava.API {


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
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
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
     * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
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