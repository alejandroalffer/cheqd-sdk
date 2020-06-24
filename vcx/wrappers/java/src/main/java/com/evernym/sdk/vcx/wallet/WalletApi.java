package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class WalletApi extends VcxJava.API {
    private static final Logger logger = LoggerFactory.getLogger("WalletApi");

    private WalletApi() {
    }

    private static Callback vcxExportWalletCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int exportHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], exportHandle = [" + exportHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = exportHandle;
            future.complete(result);
        }
    };

    /**
     * Exports opened wallet
     *
     * @param  exportPath       Path to export wallet to User's File System.
     * @param  encryptionKey    String representing the User's Key for securing (encrypting) the exported Wallet.
     *
     * @return                  void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> exportWallet(
            String exportPath,
            String encryptionKey
    ) throws VcxException {
        ParamGuard.notNull(exportPath, "exportPath");
        ParamGuard.notNull(encryptionKey, "encryptionKey");
        logger.debug("exportWallet() called with: exportPath = [" + exportPath + "], encryptionKey = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_export(commandHandle, exportPath, encryptionKey, vcxExportWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxImportWalletCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int importHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], importHandle = [" + importHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = importHandle;
            future.complete(result);
        }
    };

    /**
     * Creates a new secure wallet and then imports its content
     * according to fields provided in import_config
     * Cannot be used if wallet is already opened (Especially if vcx_init has already been used).
     *
     * @param  config           Configuration JSON for importing wallet
     *                          "{"wallet_name":"","wallet_key":"","exported_wallet_path":"","backup_key":"","key_derivation":""}"
     *
     * @return                  void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> importWallet(
            String config
    ) throws VcxException {
        ParamGuard.notNull(config, "config");
        logger.debug("importWallet() called with: config = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_import(commandHandle, config, vcxImportWalletCB);
        checkResult(result);

        return future;
    }

    /**
     * Callback used when bytesCb completes.
     */
    private static Callback signWithPaymentAddressCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, Pointer arr_raw, int arr_len) {

            CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
            if (! checkCallback(future, err)) return;

            byte[] result = new byte[arr_len];
            arr_raw.read(0, result, 0, arr_len);
            future.complete(result);
        }
    };

    /**
     * Signs a message with a payment address.
     *
     * @param address:  Payment address of message signer.
     * @param message   The message to be signed
     *
     * @return A future that resolves to a signature string.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<byte[]> signWithAddress(
            String address,
            byte[] message) throws VcxException {

        ParamGuard.notNullOrWhiteSpace(address, "address");
        ParamGuard.notNull(message, "message");

        CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_sign_with_address(
                commandHandle,
                address,
                message,
                message.length,
                signWithPaymentAddressCb);

        checkResult(result);

        return future;
    }

    /**
     * Callback used when boolCb completes.
     */
    private static Callback verifyWithAddressCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, boolean valid) {

            CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
            if (! checkCallback(future, err)) return;

            Boolean result = valid;
            future.complete(result);
        }
    };

    /**
     * Verify a signature with a payment address.
     *
     * @param address   Payment address of the message signer
     * @param message   Message that has been signed
     * @param signature A signature to be verified
     * @return A future that resolves to true if signature is valid, otherwise false.
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Boolean> verifyWithAddress(
            String address,
            byte[] message,
            byte[] signature) throws VcxException {

        ParamGuard.notNullOrWhiteSpace(address, "address");
        ParamGuard.notNull(message, "message");
        ParamGuard.notNull(signature, "signature");

        CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_verify_with_address(
                commandHandle,
                address,
                message,
                message.length,
                signature,
                signature.length,
                verifyWithAddressCb);

        checkResult(result);

        return future;
    }

    private static Callback vcxAddRecordWalletCB = new Callback() {
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
     * Adds a record to the wallet
     *
     * @param recordType        type of record. (e.g. 'data', 'string', 'foobar', 'image')
     * @param recordId          the id ("key") of the record.
     * @param recordValue       value of the record with the associated id.
     *
     * @return                  void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> addRecordWallet(
            String recordType,
            String recordId,
            String recordValue
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(recordValue, "recordValue");
        logger.debug("addRecordWallet() called with: recordType = [" + recordType + "], recordId = [" + recordId + "], recordValue = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        String recordTag = "{}";

        int result = LibVcx.api.vcx_wallet_add_record(commandHandle, recordType, recordId, recordValue, recordTag, vcxAddRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxDeleteRecordWalletCB = new Callback() {
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
     * Deletes an existing record.
     *
     * @param recordType        type of record. (e.g. 'data', 'string', 'foobar', 'image')
     * @param recordId          the id ("key") of the record.
     *
     * @return                  void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> deleteRecordWallet(
            String recordType,
            String recordId
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        logger.debug("deleteRecordWallet() called with: recordType = [" + recordType + "], recordId = [" + recordId + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_delete_record(commandHandle, recordType, recordId, vcxDeleteRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxGetRecordWalletCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String recordValue) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], recordValue = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // if nonzero errorcode, ignore walletHandle (null)
            // if error fail
            // if error = 0 then send the result
            future.complete(recordValue);
        }
    };

    /**
     * Gets a record from Wallet.
     *
     * @param recordType        type of record. (e.g. 'data', 'string', 'foobar', 'image')
     * @param recordId          the id ("key") of the record.
     *
     * @return                  received record as JSON
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<String> getRecordWallet(
            String recordType,
            String recordId,
            String optionsJson
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(optionsJson, "optionsJson");
        logger.debug("getRecordWallet() called with: recordType = [" + recordType + "], recordId = [" + recordId + "], optionsJson = [" + optionsJson + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        if (optionsJson.isEmpty()) optionsJson = "{}";

        int result = LibVcx.api.vcx_wallet_get_record(commandHandle, recordType, recordId, optionsJson, vcxGetRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxUpdateRecordWalletCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    /**
     * Updates the value of a record already in the wallet.
     *
     * @param recordType        type of record. (e.g. 'data', 'string', 'foobar', 'image')
     * @param recordId          the id ("key") of the record.
     * @param recordValue       new value of the record with the associated id.
     *
     * @return                  void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> updateRecordWallet(
            String recordType,
            String recordId,
            String recordValue
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(recordValue, "recordValue");
        logger.debug("updateRecordWallet() called with: recordType = [" + recordType + "], recordId = [" + recordId + "], recordValue = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_update_record_value(commandHandle, recordType, recordId, recordValue, vcxUpdateRecordWalletCB);
        checkResult(result);

        return future;
    }

    public static void setWalletHandle(int handle) {
        LibVcx.api.vcx_wallet_set_handle(handle);
    }

    // vcx_error_t vcx_wallet_backup_create(vcx_command_handle_t command_handle, const char *source_id,
    //                                       void (*cb)(vcx_command_handle_t, vcx_error_t, vcx_wallet_backup_handle_t));
    private static Callback vcxCreateWalletBackupCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int walletHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], walletHandle = [" + walletHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = walletHandle;
            future.complete(result);
        }
    };

    /**
     * Create a Wallet Backup object that provides a Cloud wallet backup and provision's backup protocol with Agent
     *
     * @param sourceID        institution's personal identification for the user
     * @param backupKey       String representing the User's Key for securing (encrypting) the exported Wallet.
     *
     * @return                handle that should be used to perform actions with the WalletBackup object.
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> createWalletBackup(
        String sourceID,
        String backupKey
    ) throws VcxException {
        ParamGuard.notNull(sourceID, "sourceID");
        ParamGuard.notNull(backupKey, "backupKey ");
        logger.debug("createWalletBackup() called with: sourceID = [" + sourceID + "], backupKey = [" + backupKey + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_create(commandHandle, sourceID, backupKey, vcxCreateWalletBackupCB);
        checkResult(result);

        return future;

    }

    // vcx_error_t vcx_wallet_backup_backup(vcx_command_handle_t command_handle, vcx_wallet_backup_handle_t wallet_backup_handle, const char *path, const char *backup_key,
    //                                   void (*cb)(vcx_command_handle_t, vcx_error_t));
    private static Callback vcxBackupWalletBackupBackupCB = new Callback() {
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
     * Wallet Backup to the Cloud
     *
     * @param  walletBackupHandle  handle pointing to WalletBackup object.
     * @param path                 path to export wallet to User's File System. (This instance of the export
     *
     * @return                     void
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> backupWalletBackup(
            int walletBackupHandle,
            String path
    ) throws VcxException {
        ParamGuard.notNull(walletBackupHandle, "walletBackupHandle");
        ParamGuard.notNull(path, "path");
        logger.debug("backupWalletBackup() called with: walletBackupHandle = [" + walletBackupHandle + "], path = [" + path + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_backup(commandHandle, walletBackupHandle, path, vcxBackupWalletBackupBackupCB);
        checkResult(result);

        return future;
    }

    // vcx_error_t vcx_wallet_backup_update_state(vcx_command_handle_t command_handle, vcx_wallet_backup_handle_t wallet_backup_handle,
    //                                     void (*cb)(vcx_command_handle_t, vcx_error_t, vcx_state_t));
    private static Callback vcxUpdateWalletBackupStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state);
        }
    };

    /**
     * Checks for any state change and updates the the state attribute
     *
     * @param  walletBackupHandle  handle pointing to WalletBackup object.
     *
     * @return                     the most current state of the WalletBackup object
     *
     * @throws VcxException Thrown if an error occurs when calling the underlying SDK.
     */
    public static CompletableFuture<Integer> updateWalletBackupState(
        int walletBackupHandle  // is this a int?
    )  throws VcxException {
        ParamGuard.notNull(walletBackupHandle, "walletBackupHandle");
        logger.debug("updateWalletBackupState() called with: walletBackupHandle = [" + walletBackupHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_update_state(commandHandle, walletBackupHandle, vcxUpdateWalletBackupStateCB);
        checkResult(result);

        return future;

    }
    // vcx_error_t vcx_wallet_backup_update_state_with_message(vcx_command_handle_t command_handle, vcx_wallet_backup_handle_t wallet_backup_handle, const char *message,
    //                                                     void (*cb)(vcx_command_handle_t, vcx_error_t, vcx_state_t));
    private static Callback vcxUpdateWalletBackupStateWithMessageCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return; //TODO: check if we need to add more params here
            future.complete(state);
        }
    };

    /**
     * Update the state of the WalletBackup object based on the given message.
     *
     * @param  walletBackupHandle  handle pointing to WalletBackup object.
     * @param  message              message to process for any WalletBackup state transitions.
     *
     * @return                      the most current state of the WalletBackup object.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> updateWalletBackupStateWithMessage(
        int walletBackupHandle, // is this a int?
        String message
    )  throws VcxException {
        ParamGuard.notNull(walletBackupHandle, "walletBackupHandle");
        ParamGuard.notNull(message, "message");
        logger.debug("updateWalletBackupState() called with: walletBackupHandle = [" + walletBackupHandle + "], message = [" + message + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_update_state_with_message(commandHandle, walletBackupHandle, message, vcxUpdateWalletBackupStateWithMessageCB);
        checkResult(result);

        return future;

    }


    // vcx_error_t vcx_wallet_backup_serialize(vcx_command_handle_t command_handle, vcx_wallet_backup_handle_t wallet_backup_handle,
    //                                     void (*cb)(vcx_command_handle_t, vcx_error_t, const char*));
    private static Callback vcxWalletBackupSerializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String data) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], data = [" + data + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return; //TODO: check if we need to add more params here
            future.complete(data);
        }
    };


    /**
     * Get JSON string representation of WalletBackup object.
     *
     * @param  walletBackupHandle  handle pointing to WalletBackup object.
     *
     * @return                      WalletBackup object as JSON string.
     *
     * @throws VcxException         If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> serializeBackupWallet(
        int walletBackupHandle // is this a int?
    )  throws VcxException {
        ParamGuard.notNull(walletBackupHandle, "walletBackupHandle");
        logger.debug("serializeBackupWallet() called with: walletBackupHandle = [" + walletBackupHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_serialize(commandHandle, walletBackupHandle, vcxWalletBackupSerializeCB);
        checkResult(result);

        return future;

    }


    // vcx_error_t vcx_wallet_backup_deserialize(vcx_command_handle_t command_handle, const char *wallet_backup_str,
    //                                       void (*cb)(vcx_command_handle_t, vcx_error_t, vcx_wallet_backup_handle_t));
    private static Callback vcxWalletBackupDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int walletBackupHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], walletBackupHandle = [" + walletBackupHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(walletBackupHandle);
        }
    };

    /**
     * Takes a json string representing a WalletBackup object and recreates an object matching the JSON.
     *
     * @param  walletBackupStr JSON string representing a WalletBackup object.
     *
     * @return                 handle that should be used to perform actions with the WalletBackup object.
     *
     * @throws VcxException    If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> deserializeBackupWallet(
        String walletBackupStr
    )  throws VcxException {
        ParamGuard.notNull(walletBackupStr, "walletBackupStr");
        logger.debug("deserializeBackupWallet() called with: walletBackupStr = [" + walletBackupStr + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_deserialize(commandHandle, walletBackupStr, vcxWalletBackupDeserializeCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxBackupRestore = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(0);
        }
    };

    /**
     * Requests a recovery of a backup previously stored with a cloud agent
     *
     * @param  config          config to use for wallet backup restoring
     *                         "{"wallet_name":"","wallet_key":"","exported_wallet_path":"","backup_key":"","key_derivation":""}"
     *
     * @return                 void
     *
     * @throws VcxException    If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> restoreWalletBackup(
            String config
    ) throws VcxException {
        ParamGuard.notNull(config, "config");
        logger.debug("restoreBackup() called with: config = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_backup_restore(commandHandle, config, vcxBackupRestore);
        checkResult(result);

        return future;
    }

}
