package com.evernym.vdrtools.vdr;

import com.evernym.vdrtools.IndyException;
import com.evernym.vdrtools.IndyJava;
import com.evernym.vdrtools.LibIndy;
import com.evernym.vdrtools.ParamGuard;
import com.sun.jna.Callback;

import java.util.concurrent.CompletableFuture;


public class VDR extends IndyJava.API {

    private VDR() {

    }

    private static Callback stringCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String str) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            String result = str;
            future.complete(result);
        }
    };

    private static Callback voidCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err) {

            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            Void result = null;
            future.complete(result);
        }
    };


    public static CompletableFuture<Void> registerIndyLedger(
            String namespaceList,
            String genesisTxnData,
            String taaConfig
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(genesisTxnData, "genesisTxnData");
        ParamGuard.notNull(taaConfig, "taaConfig");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_register_indy_ledger(
                commandHandle,
                namespaceList,
                genesisTxnData,
                taaConfig,
                voidCb);

        checkResult(future, result);

        return future;
    }

    public static CompletableFuture<Void> registerCheqdLedger(
            String namespaceList,
            String chainId,
            String nodeAddrList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(chainId, "chainId");
        ParamGuard.notNull(nodeAddrList, "nodeAddrList");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_register_cheqd_ledger(
                commandHandle,
                namespaceList,
                chainId,
                nodeAddrList,
                voidCb);

        checkResult(future, result);

        return future;
    }

    public static CompletableFuture<String> ping(
            String namespaceList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_ping(
                commandHandle,
                namespaceList,
                stringCb);

        checkResult(future, result);

        return future;
    }

    public static CompletableFuture<Void> cleanup() throws IndyException {
        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_cleanup(
                commandHandle,
                voidCb);

        checkResult(future, result);

        return future;
    }

    // Write functions
    // Todo: move to corresponding classes

    public static CompletableFuture<String> resolveDID(
            String fqDID,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqDID, "fqDID");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_resolve_did(
                commandHandle,
                fqDID,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    public static CompletableFuture<String> resolveSchema(
            String fqSchema,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqSchema, "fqSchema");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_resolve_schema(
                commandHandle,
                fqSchema,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    public static CompletableFuture<String> resloveCredDef(
            String fqCredDef,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqCredDef, "fqCredDef");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_resolve_cred_def(
                commandHandle,
                fqCredDef,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }
}
