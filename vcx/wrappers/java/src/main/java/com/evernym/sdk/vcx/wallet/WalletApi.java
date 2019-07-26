package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
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
    public static CompletableFuture<Integer> backupWalletBackup(
        int walletBackupHandle, // is this a int?
        String path,
    )  throws VcxException {
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
}
