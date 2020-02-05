package com.evernym.sdk.vcx.vcx;


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.*;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class VcxApi extends VcxJava.API {
    private static final Logger logger = LoggerFactory.getLogger("VcxApi");
    private VcxApi() {
    }

    public static int initSovToken() throws VcxException {
        logger.debug("initSovToken()");
        int result = LibVcx.api.sovtoken_init();
        checkResult(result);
        return result;
    }

//     public static int initNullPay() throws VcxException {
//         logger.debug("initNullPay()");
//         int result = LibVcx.api.nullpay_init();
//         checkResult(result);
//         return result;
//     }

    private static Callback vcxIniWithConfigCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = err;
            future.complete(result);
        }
    };

    private static Callback vcxInitCB = new Callback() {


        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err) {
            logger.debug("callback() called with: xcommandHandle = [" + xcommandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;
            int result = err;
            future.complete(result);

        }
    };

    public static CompletableFuture<Integer> vcxInitWithConfig(String configJson) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configJson, "config");
        logger.debug("vcxInitWithConfig() called with: configJson = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init_with_config(
                commandHandle,
                configJson,
                vcxIniWithConfigCB);
        checkResult(result);

        return future;

    }

    public static CompletableFuture<Integer> vcxInit(String configPath) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configPath, "configPath");
        logger.debug("vcxInit() called with: configPath = [" + configPath + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init(
                commandHandle, configPath,
                vcxInitCB);
        checkResult(result);
        return future;
    }

    public static int vcxInitMinimal(String configJson) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configJson, "config");
        logger.debug("vcxInitMinimal() called with: configJson = [" + configJson + "]");

        int result = LibVcx.api.vcx_init_minimal(
                configJson);
        checkResult(result);

        return result;
    }

    public static int vcxShutdown(Boolean deleteWallet) throws VcxException {
        logger.debug("vcxShutdown() called with: deleteWallet = [" + deleteWallet + "]");
        int result = LibVcx.api.vcx_shutdown(deleteWallet);
        checkResult(result);
        return result;
    }

    public static String vcxVersion() throws VcxException {
        logger.debug("vcxVersion()");
        return LibVcx.api.vcx_version();
    }

    public static String vcxErrorCMessage(int errorCode) {
        logger.debug("vcxErrorCMessage() called with: errorCode = [" + errorCode + "]");
        return LibVcx.api.vcx_error_c_message(errorCode);

    }

    public static void logMessage(String loggerName, int level, String message) {
        LibVcx.logMessage(loggerName, level, message);
    }

    public static int vcxSetLogger(Pointer context, Callback enabled, Callback log, Callback flush) throws VcxException {
        logger.debug("vcxSetLogger()");
        int result = LibVcx.api.vcx_set_logger(context, enabled, log, flush);
        checkResult(result);
        return result;
    }

    public static int vcxSetDefaultLogger(String logLevel) throws VcxException {
        logger.debug("vcxSetDefaultLogger()");
        int result = LibVcx.api.vcx_set_default_logger(logLevel);
        checkResult(result);
        return result;
    }

}
