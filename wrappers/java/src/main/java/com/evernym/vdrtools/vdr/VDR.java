package com.evernym.vdrtools.vdr;

import com.evernym.vdrtools.IndyException;
import com.evernym.vdrtools.IndyJava;
import com.evernym.vdrtools.LibIndy;
import com.evernym.vdrtools.ParamGuard;
import com.sun.jna.Callback;
import com.sun.jna.Pointer;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;


public class VDR extends IndyJava.API implements AutoCloseable {

    private final int vdrHandle;

    private VDR(int vdrHandle) {
        this.vdrHandle = vdrHandle;
    }

    public int getVdrHandle() {
        return vdrHandle;
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

    private static Callback createVdrCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, int vdr_handle) {

            CompletableFuture<VDR> future = (CompletableFuture<VDR>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            VDR result = new VDR(vdr_handle);
            future.complete(result);
        }
    };

    private static Callback prepareTxnCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String namespace, String signature_spec,Pointer txn_bytes_raw, int txn_bytes_len, Pointer bytes_to_sign_raw, int bytes_to_sign_len, String endorsement_spec) {

            CompletableFuture<VdrResults.PreparedTxnResult> future = (CompletableFuture<VdrResults.PreparedTxnResult>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            byte[] txnBytes = new byte[bytes_to_sign_len];
            bytes_to_sign_raw.read(0, txnBytes, 0, bytes_to_sign_len);

            byte[] bytesToSign = new byte[bytes_to_sign_len];
            bytes_to_sign_raw.read(0, bytesToSign, 0, bytes_to_sign_len);

            VdrResults.PreparedTxnResult result = new VdrResults.PreparedTxnResult(namespace, signature_spec,txnBytes, bytesToSign, endorsement_spec);
            future.complete(result);
        }
    };

    public static CompletableFuture<VDR> createVDR() throws IndyException {
        CompletableFuture<VDR> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.vdr_create(
                commandHandle,
                createVdrCb
        );

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> registerIndyLedger(
            VDR vdr,
            String namespaceList,
            String genesisTxnData,
            String taaConfig
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(genesisTxnData, "genesisTxnData");
        ParamGuard.notNull(taaConfig, "taaConfig");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_register_indy_ledger(
                commandHandle,
                handle,
                namespaceList,
                genesisTxnData,
                taaConfig,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> registerCheqdLedger(
            VDR vdr,
            String namespaceList,
            String chainId,
            String nodeAddrList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(chainId, "chainId");
        ParamGuard.notNull(nodeAddrList, "nodeAddrList");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_register_cheqd_ledger(
                commandHandle,
                handle,
                namespaceList,
                chainId,
                nodeAddrList,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> ping(
            VDR vdr,
            String namespaceList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_ping(
                commandHandle,
                handle,
                namespaceList,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> cleanup(
            VDR vdr
    ) throws IndyException {
        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();
        int result = LibIndy.api.vdr_cleanup(
                commandHandle,
                handle,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resolveDID(
            VDR vdr,
            String fqDID,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqDID, "fqDID");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_resolve_did(
                commandHandle,
                handle,
                fqDID,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resolveSchema(
            VDR vdr,
            String fqSchema,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqSchema, "fqSchema");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_resolve_schema(
                commandHandle,
                handle,
                fqSchema,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resloveCredDef(
            VDR vdr,
            String fqCredDef,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqCredDef, "fqCredDef");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_resolve_cred_def(
                commandHandle,
                handle,
                fqCredDef,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }


    private static CompletableFuture<VdrResults.PreparedTxnResult> prepareDID(
            VDR vdr,
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        ParamGuard.notNull(txnSpecificParams, "txnSpecificParams");
        ParamGuard.notNull(submitterDID, "submitterDID");

        CompletableFuture<VdrResults.PreparedTxnResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_prepare_did(
                commandHandle,
                handle,
                txnSpecificParams,
                submitterDID,
                endorser,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<VdrResults.PreparedTxnResult> prepareSchema(
            VDR vdr,
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        ParamGuard.notNull(txnSpecificParams, "txnSpecificParams");
        ParamGuard.notNull(submitterDID, "submitterDID");

        CompletableFuture<VdrResults.PreparedTxnResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_prepare_schema(
                commandHandle,
                handle,
                txnSpecificParams,
                submitterDID,
                endorser,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<VdrResults.PreparedTxnResult> prepareCredDef(
            VDR vdr,
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        ParamGuard.notNull(txnSpecificParams, "txnSpecificParams");
        ParamGuard.notNull(submitterDID, "submitterDID");

        CompletableFuture<VdrResults.PreparedTxnResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_prepare_cred_def(
                commandHandle,
                handle,
                txnSpecificParams,
                submitterDID,
                endorser,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> submitTxn(
            VDR vdr,
            VdrResults.PreparedTxnResult preparedTxn,
            byte[] signature
    ) throws IndyException {
        ParamGuard.notNull(preparedTxn, "preparedTxn");
        ParamGuard.notNull(signature, "signature");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_submit_txn(
                commandHandle,
                handle,
                preparedTxn.getNamespace(),
                preparedTxn.getSignatureSpec(),
                preparedTxn.getTxnBytes(),
                preparedTxn.getTxnBytes().length,
                signature,
                signature.length,
                preparedTxn.getEndorsementSpec(),
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> submitQuery(
            VDR vdr,
            String namespace,
            String query
    ) throws IndyException {
        ParamGuard.notNull(namespace, "namespace");
        ParamGuard.notNull(query, "query");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.vdr_submit_query(
                commandHandle,
                handle,
                namespace,
                query,
                stringCb
        );

        checkResult(future,result);

        return future;
    }


    public CompletableFuture<Void> registerIndyLedger(
            String namespaceList,
            String genesisTxnData,
            String taaConfig
    ) throws IndyException {
        return registerIndyLedger(this, namespaceList, genesisTxnData, taaConfig);
    }


    public CompletableFuture<Void> registerCheqdLedger(
            String namespaceList,
            String chainId,
            String nodeAddrList
    ) throws IndyException {
        return registerCheqdLedger(this, namespaceList, chainId, nodeAddrList);
    }

    public CompletableFuture<String> ping(
            String namespaceList
    ) throws IndyException {
        return ping(this, namespaceList);
    }

    public CompletableFuture<Void> cleanup() throws IndyException {
        return cleanup(this);
    }

    public CompletableFuture<String> resolveDID(
            String fqDID,
            String cacheOptions
    ) throws IndyException {
        return resolveDID(this, fqDID, cacheOptions);
    }

    public CompletableFuture<String> resolveSchema(
            String fqSchema,
            String cacheOptions
    ) throws IndyException {
        return resolveSchema(this, fqSchema, cacheOptions);
    }

    public CompletableFuture<String> resloveCredDef(
            String fqCredDef,
            String cacheOptions
    ) throws IndyException {
        return resloveCredDef(this, fqCredDef, cacheOptions);
    }

    public CompletableFuture<VdrResults.PreparedTxnResult> prepareDID(
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        return prepareDID(this, txnSpecificParams, submitterDID, endorser);
    }

    public CompletableFuture<VdrResults.PreparedTxnResult> prepareSchema(
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        return prepareSchema(this, txnSpecificParams, submitterDID, endorser);
    }

    public CompletableFuture<VdrResults.PreparedTxnResult> prepareCredDef(
            String txnSpecificParams,
            String submitterDID,
            String endorser
    ) throws IndyException {
        return prepareCredDef(this, txnSpecificParams, submitterDID, endorser);
    }

    public CompletableFuture<String> submitTxn(
            VdrResults.PreparedTxnResult preparedTxn,
            byte[] signature
    ) throws IndyException {
        return submitTxn(this, preparedTxn, signature);
    }

    public CompletableFuture<String> submitQuery(
            String namespace,
            String query
    ) throws IndyException {
        return submitQuery(this, namespace, query);
    }

    @Override
    public void close() throws InterruptedException, ExecutionException, IndyException {
        cleanup().get();
    }
}
