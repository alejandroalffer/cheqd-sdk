package com.evernym.sdk.vcx.schema;


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class SchemaApi extends VcxJava.API {
    private static final Logger logger = LoggerFactory.getLogger("SchemaApi");

    private static Callback schemaCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int schemaHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = schemaHandle;
            future.complete(result);
        }
    };

	/**
	 * Create a new Schema object and publish corresponding record on the ledger
	 *
	 * @param  sourceId             Enterprise's personal identification for the Schema.
	 * @param  schemaName           Name of schema
	 * @param  version              Version of schema. A semver-compatible value like "1.0" is encouraged.
	 * @param  data                 A list of attributes that will make up the schema, represented as a string containing a JSON array.
	 *                              The number of attributes should be less or equal to 125, because larger arrays cause various downstream problems.
	 *                              This limitation is an annoyance that we'd like to remove.
	 *                              "["attr1", "attr2", "attr3"]"
	 * @param  paymentHandle        unused parameter (pass 0)
	 *
	 * @return                      handle that should be used to perform actions with the Schema object.
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<Integer> schemaCreate(String sourceId,
                                                          String schemaName,
                                                          String version,
                                                          String data,
                                                          int paymentHandle) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(schemaName, "schemaName");
        ParamGuard.notNullOrWhiteSpace(version, "version");
        ParamGuard.notNullOrWhiteSpace(data, "data");
        logger.debug("schemaCreate() called with: sourceId = [" + sourceId + "], schemaName = [****], version = [****]" + " data = <****>" + " payment_handle = <" + paymentHandle + ">");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_schema_create(
                commandHandle,
                sourceId,
                schemaName,
                version,
                data,
                paymentHandle,
                schemaCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaSerializeHandle = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String serializedData) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], serializedData = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = serializedData;
            future.complete(result);
        }
    };

	/**
	 * Get JSON string representation of Schema object.
	 *
	 * @param  schemaHandle     handle pointing to a Schema object.
	 *
	 * @return                  Schema object as JSON string.
	 *
	 * @throws VcxException     If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<String> schemaSerialize(int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        logger.debug("schemaSerialize() called with: schemaHandle = [" + schemaHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_schema_serialize(
                commandHandle,
                schemaHandle,
                schemaSerializeHandle
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int schemaHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = schemaHandle;
            future.complete(result);
        }
    };

	/**
	 * Takes a json string representing a Schema object and recreates an object matching the JSON.
	 *
	 * @param  schemaData           JSON string representing a Schema object.
	 *
	 * @return                      handle that should be used to perform actions with the Schema object.
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<Integer> schemaDeserialize(String schemaData) throws VcxException {
        ParamGuard.notNull(schemaData, "schemaData");
        logger.debug("schemaDeserialize() called with: schemaData = [****]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api().vcx_schema_deserialize(
                commandHandle,
                schemaData,
                schemaDeserializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaGetAttributesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err,int schemaHandle, String schemaAttributes) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle +  "],  schemaAttributes = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaAttributes);
        }
    };

	/**
	 * Retrieves all of the data associated with a schema on the ledger.
	 *
	 * @param  sourceId         Enterprise's personal identification for the user.
	 * @param  schemaId         id of schema to get the list of attributes
	 *
	 * @return                  JSON string representing all of the data of a schema already on the ledger
	 *
	 * @throws VcxException     If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<String> schemaGetAttributes( String sourceId, String schemaId) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        logger.debug("schemaGetAttributes() called with: sourceId = [" + sourceId + "], schemaHandle = [****]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api().vcx_schema_get_attributes(commandHandle, sourceId,schemaId, schemaGetAttributesCB);
        checkResult(result);
        return future;
    }

    private static Callback schemaGetSchemaID = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String schemaId) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaId = [****]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaId);
        }
    };

	/**
	 * Retrieves id of Schema object
	 *
	 * @param  schemaHandle     handle pointing to a Schema object.
	 *
	 * @return                  if of Schema object
	 *
	 * @throws VcxException     If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<String> schemaGetSchemaId( int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "SchemaHandle");
        logger.debug("schemaGetSchemaId() called with: schemaHandle = [" + schemaHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api().vcx_schema_get_schema_id(commandHandle,schemaHandle, schemaGetSchemaID);
        checkResult(result);
        return future;
    }

	/**
	 * Releases the Schema object by de-allocating memory
	 *
	 * @param  schemaHandle         handle pointing to a Schema object.
	 *
	 * @return                      void
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
    public static int schemaRelease(
            int schemaHandle
    ) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        logger.debug("schemaRelease() called with: schemaHandle = [" + schemaHandle + "]");

        int result = LibVcx.api().vcx_schema_release(schemaHandle);
        checkResult(result);

        return result;
    }

    private static Callback schemaPrepareForEndorserCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int handle, String transaction) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], handle = [" + handle + "], transaction = [****]");
            CompletableFuture<SchemaPrepareForEndorserResult> future = (CompletableFuture<SchemaPrepareForEndorserResult>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            SchemaPrepareForEndorserResult result = new SchemaPrepareForEndorserResult(handle, transaction);
            future.complete(result);
        }
    };

	/**
	 * Create a new Schema object that will be published by Endorser later.
	 * <p>
	 * Note that Schema can't be used for credential issuing until it will be published on the ledger.
	 *
	 * @param  sourceId             Enterprise's personal identification for the user.
	 * @param  schemaName           Name of schema
	 * @param  version              Version of schema. A semver-compatible value like "1.0" is encouraged.
	 * @param  data                 A list of attributes that will make up the schema, represented as a string containing a JSON array.
	 *                              The number of attributes should be less or equal to 125, because larger arrays cause various downstream problems.
	 *                              This limitation is an annoyance that we'd like to remove.
	 *                              "["attr1", "attr2", "attr3"]"
	 * @param  endorser             DID of the Endorser that will submit the transaction.
	 *
	 * @return                      handle that should be used to perform actions with the Schema object.
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
    public static CompletableFuture<SchemaPrepareForEndorserResult> schemaPrepareForEndorser(String sourceId,
                                                                                             String schemaName,
                                                                                             String version,
                                                                                             String data,
                                                                                             String endorser) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNull(schemaName, "schemaName");
        ParamGuard.notNull(version, "version");
        ParamGuard.notNull(data, "data");
        ParamGuard.notNull(endorser, "endorserendorser");
	    logger.debug("schemaCreate() called with: sourceId = [" + sourceId + "], schemaName = [****], version = [****]" + " data = <****>" + " endorser = <****>");
        CompletableFuture<SchemaPrepareForEndorserResult> future = new CompletableFuture<SchemaPrepareForEndorserResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_prepare_for_endorser(
                commandHandle,
		        sourceId,
		        schemaName,
		        version,
		        data,
		        endorser,
		        schemaPrepareForEndorserCB);
        checkResult(result);

        return future;
    }

	private static Callback vcxIntegerCB = new Callback() {
		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int commandHandle, int err, int s) {
			logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], s = [" + s + "]");
			CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
			if (!checkCallback(future, err)) return;
			Integer result = s;
			future.complete(result);
		}
	};

	/**
	 * Checks if schema is published on the Ledger and updates the state.
	 *
	 * @param  schemaHandle         handle pointing to a Schema object.
	 *
	 * @return                      the most current state of Schema object.
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
	public static CompletableFuture<Integer> schemaUpdateState(int schemaHandle) throws VcxException {
		logger.debug("vcxSchemaUpdateState() called with: schemaHandle = [" + schemaHandle + "]");
		CompletableFuture<Integer> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.vcx_schema_update_state(
				commandHandle,
				schemaHandle,
				vcxIntegerCB
		);
		checkResult(result);
		return future;
	}

	/**
	 * Get the current state of the Schema object
	 * Schema states:
	 *     0 - Built
	 *     1 - Published
	 *
	 * @param  schemaHandle         handle pointing to a Schema object.
	 *
	 * @return                      the most current state of the Schema object.
	 *
	 * @throws VcxException         If an exception occurred in Libvcx library.
	 */
	public static CompletableFuture<Integer> schemaGetState(int schemaHandle) throws VcxException {
		logger.debug("schemaGetState() called with: schemaHandle = [" + schemaHandle + "]");
		CompletableFuture<Integer> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.vcx_schema_get_state(
				commandHandle,
				schemaHandle,
				vcxIntegerCB
		);
		checkResult(result);
		return future;
	}
}
