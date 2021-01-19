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
        int result = LibVcx.api().sovtoken_init();
        checkResult(result);
        return result;
    }

//     public static int initNullPay() throws VcxException {
//         logger.debug("initNullPay()");
//         int result = LibVcx.api().nullpay_init();
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

    /**
     * Initializes VCX with config
     * An example file is at libvcx/sample_config/config.json
     * The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md
     *
     * @param  configJson       config as JSON string to use for library initialization
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> vcxInitWithConfig(String configJson) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configJson, "config");
        logger.debug("vcxInitWithConfig() called with: configJson = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_init_with_config(
                commandHandle,
                configJson,
                vcxIniWithConfigCB);
        checkResult(result);

        return future;

    }

    /**
     * Initializes VCX with config file
     * An example file is at libvcx/sample_config/config.json
     * The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md
     *
     * @param  configPath       path to config file to use for library initialization
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Integer> vcxInit(String configPath) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configPath, "configPath");
        logger.debug("vcxInit() called with: configPath = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_init(
                commandHandle, configPath,
                vcxInitCB);
        checkResult(result);
        return future;
    }

    /**
     * Initialize vcx with the minimal configuration (wallet, pool must already be set with)
     *
     * @param  configJson       minimal configuration as JSON string
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static int vcxInitMinimal(String configJson) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configJson, "config");
        logger.debug("vcxInitMinimal() called with: configJson = [****]");

        int result = LibVcx.api().vcx_init_minimal(
                configJson);
        checkResult(result);

        return result;
    }
    private static Callback vcxInitPoolCB = new Callback() {


        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err) {
            logger.debug("callback() called with: xcommandHandle = [" + xcommandHandle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(null);

        }
    };

    /**
     * Connect to a Pool Ledger
     *
     * You can deffer connecting to the Pool Ledger during library initialization (vcx_init or vcx_init_with_config)
     * to decrease the taken time by omitting `genesis_path` field in config JSON.
     * Next, you can use this function (for instance as a background task) to perform a connection to the Pool Ledger.
     *
     * Note: Pool must be already initialized before sending any request to the Ledger.
     *
     * EXPERIMENTAL
     *
     * @param  poolConfig       the configuration JSON containing pool related settings:
     *                          {
     *                              genesis_path: string - path to pool ledger genesis transactions,
     *                              pool_name: Optional[string] - name of the pool ledger configuration will be created.
     *                                                   If no value specified, the default pool name pool_name will be used.
     *                              pool_config: Optional[string] - runtime pool configuration json:
     *                                  {
     *                                      "timeout": int (optional), timeout for network request (in sec).
     *                                      "extended_timeout": int (optional), extended timeout for network request (in sec).
     *                                      "preordered_nodes": array<string> -  (optional), names of nodes which will have a priority during request sending:
     *                                         ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
     *                                         This can be useful if a user prefers querying specific nodes.
     *                                         Assume that `Node1` and `Node2` nodes reply faster.
     *                                         If you pass them Libindy always sends a read request to these nodes first and only then (if not enough) to others.
     *                                         Note: Nodes not specified will be placed randomly.
     *                                      "number_read_nodes": int (optional) - the number of nodes to send read requests (2 by default)
     *                                         By default Libindy sends a read requests to 2 nodes in the pool.
     *                                  }
     *                          }
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static CompletableFuture<Void> vcxInitPool(String poolConfig) throws VcxException {
        logger.debug("vcxInitPool() called with: poolConfig = [{}]", poolConfig);
        ParamGuard.notNull(poolConfig, "poolConfig");
        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_init_pool(
                commandHandle,
                poolConfig,
                vcxInitPoolCB);
        checkResult(result);

        return future;
    }
    
    /**
     * Reset libvcx to a pre-configured state, releasing/deleting any handles and freeing memory
     *
     * @param  deleteWallet     specify whether wallet/pool should be deleted
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static int vcxShutdown(Boolean deleteWallet) throws VcxException {
        logger.debug("vcxShutdown() called with: deleteWallet = [" + deleteWallet + "]");
        int result = LibVcx.api().vcx_shutdown(deleteWallet);
        checkResult(result);
        return result;
    }

    public static String vcxVersion() throws VcxException {
        logger.debug("vcxVersion()");
        return LibVcx.api().vcx_version();
    }

    public static String vcxErrorCMessage(int errorCode) {
        logger.debug("vcxErrorCMessage() called with: errorCode = [" + errorCode + "]");
        return LibVcx.api().vcx_error_c_message(errorCode);

    }

    public static void logMessage(String loggerName, int level, String message) {
        LibVcx.logMessage(loggerName, level, message);
    }

    /**
     * Set custom logger implementation.
     * Allows library user to provide custom logger implementation as set of handlers.
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static int vcxSetLogger(Pointer context, Callback enabled, Callback log, Callback flush) throws VcxException {
        logger.debug("vcxSetLogger()");
        int result = LibVcx.api().vcx_set_logger(context, enabled, log, flush);
        checkResult(result);
        return result;
    }

    /**
     * Set default logger implementation.
     * Allows library user use `env_logger` logger as default implementation.
     * More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger
     *
     * @param  logLevel         (optional) pattern that corresponds with the log messages to show.
     *
     * @return                  void
     *
     * @throws VcxException   If an exception occurred in Libvcx library.
     */
    public static int vcxSetDefaultLogger(String logLevel) throws VcxException {
        logger.debug("vcxSetDefaultLogger()");
        int result = LibVcx.api().vcx_set_default_logger(logLevel);
        checkResult(result);
        return result;
    }

}
