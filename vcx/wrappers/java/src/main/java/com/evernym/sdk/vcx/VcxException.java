package com.evernym.sdk.vcx;


import com.evernym.sdk.vcx.connection.*;
import com.evernym.sdk.vcx.credential.*;
import com.evernym.sdk.vcx.proof.*;
import com.evernym.sdk.vcx.schema.*;
import com.evernym.sdk.vcx.token.InsufficientTokenAmountException;
import com.evernym.sdk.vcx.token.InvalidPaymentAddressException;
import com.evernym.sdk.vcx.token.MissingPaymentMethodException;
import com.evernym.sdk.vcx.token.NoPaymentInformationException;
import com.evernym.sdk.vcx.utils.InvalidConfigurationException;
import com.evernym.sdk.vcx.utils.PostMsgFailureException;
import com.evernym.sdk.vcx.vcx.*;
import com.evernym.sdk.vcx.wallet.*;

import com.sun.jna.ptr.PointerByReference;
import org.json.JSONObject;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Thrown when an Indy specific error has occurred.
 */
public class VcxException extends Exception {

    private static final Logger logger = LoggerFactory.getLogger("VcxException");
    private static final long serialVersionUID = 2650355290834266234L;
    private int sdkErrorCode;
    private String  sdkMessage;
    private String  sdkFullMessage;
    private String  sdkCause;
    private String sdkBacktrace;

    /**
     * Initializes a new VcxException with the specified message.
     *
     * @param message The message for the exception.
     * @param sdkErrorCode The error code for the exception.
     */
    protected VcxException(String message, int sdkErrorCode) {
        super(message);
        this.sdkErrorCode = sdkErrorCode;
        setSdkErrorDetails();
    }

    private void setSdkErrorDetails(){
        PointerByReference errorDetailsJson = new PointerByReference();

        LibVcx.api.vcx_get_current_error(errorDetailsJson);

        try {
            JSONObject errorDetails = new JSONObject(errorDetailsJson.getValue().getString(0));
            this.sdkMessage = errorDetails.optString("error");
            this.sdkFullMessage = errorDetails.optString("message");
            this.sdkCause = errorDetails.optString("cause");
            this.sdkBacktrace = errorDetails.optString("backtrace");
        } catch(Exception e) {
           // TODO
           e.printStackTrace();
        }
    }

    /**
     * Gets the SDK error code for the exception.
     *
     * @return The SDK error code used to construct the exception.
     */
    public int getSdkErrorCode() {
        return sdkErrorCode;
    }

    /**
     * Gets the SDK error message for the exception.
     *
     * @return The SDK error message used to construct the exception.
     */
    public String  getSdkMessage() {return sdkMessage;}

    /**
     * Gets the SDK full error message for the exception.
     *
     * @return The SDK full error message used to construct the exception.
     */
    public String  getSdkFullMessage() {return sdkFullMessage;}

    /**
     * Gets the SDK error cause for the exception.
     *
     * @return The SDK error cause used to construct the exception.
     */
    public String  getSdkCause() {return sdkCause;}

    /**
     * Gets the SDK error backtrace for the exception.
     *
     * @return The SDK error backtrace used to construct the exception.
     */
    public String  getSdkBacktrace() {return sdkBacktrace;}

    /**
     * Initializes a new VcxException using the specified SDK error code.
     *
     * @param sdkErrorCode The SDK error code to construct the exception from.
     */
    static VcxException fromSdkError(int sdkErrorCode) {
        logger.debug("fromSdkError() called with: sdkErrorCode = [" + sdkErrorCode + "]");
        ErrorCode errorCode = ErrorCode.UNKNOWN_ERROR;
        try {
            errorCode = ErrorCode.valueOf(sdkErrorCode);
            if (errorCode == null) {
                errorCode = ErrorCode.UNKNOWN_ERROR;
            }
        } catch(Exception e) {
            //TODO: Log exception to logger
        }

        switch (errorCode) {
            case UNKNOWN_ERROR:
                return new UnknownErrorException();
            case INVALID_CONNECTION_HANDLE:
                return new InvalidConnectionHandleException();
            case INVALID_CONFIGURATION:
                return new InvalidConfigurationException();
            case NOT_READY:
                return new NotReadyException();
            case INVALID_OPTION:
                return new InvalidOptionException();
            case INVALID_DID:
                return new InvalidDIDException();
            case INVALID_VERKEY:
                return new InvalidVerkeyException();
            case POST_MSG_FAILURE:
                return new PostMsgFailureException();
            case INVALID_NONCE:
                return new InvalidNonceException();
            case INVALID_URL:
                return new InvalidUrlException();
            case NOT_BASE58:
                return new NotBase58Exception();
            case INVALID_ISSUER_CREDENTIAL_HANDLE:
                return new InvalidIssuerCredentialHandleException();
            case INVALID_JSON:
                return new InvalidJsonException();
            case INVALID_PROOF_HANDLE:
                return new InvalidProofHandleException();
            case INVALID_CREDENTIAL_REQUEST:
                return new InvalidCredentialRequestException();
            case INVALID_MSGPACK:
                return new InvalidMsgPackException();
            case INVALID_AGENCY_RESPONSE:
                return new InvalidAgencyResponseException();
            case INVALID_ATTRIBUTES_STRUCTURE:
                return new InvalidAttributeStructureException();
            case BIG_NUMBER_ERROR:
                return new BigNumberErrorException();
            case INVALID_PROOF:
                return new InvalidProofException();
            case INVALID_GENESIS_TXN_PATH:
                return new InvalidGenesisTxnPathException();
            case POOL_LEDGER_CONNECT:
                return new PoolLedgerConnectException();
            case CREATE_POOL_CONFIG:
                return new CreatePoolConfigException();
            case INVALID_PROOF_CREDENTIAL_DATA:
                return new InvalidProofCredentialDataException();
            case INVALID_PREDICATES_STRUCTURE:
                return new IndySubmitRequestErrorException();
            case NO_POOL_OPEN:
                return new NoPoolOpenException();
            case INVALID_SCHEMA:
                return new InvalidSchemaException();
            case CREATE_CREDENTIAL_DEF_ERR:
                return new CreateCredentialDefException();
            case UNKNOWN_LIBINDY_ERROR:
                return new UnknownLibindyErrorException();
            case CREDENTIAL_DEFINITION_NOT_FOUND:
                return new InvalidCredentialDefJsonException();
            case INVALID_CREDENTIAL_DEF_HANDLE:
                return new InvalidCredentialDefHandle();
            case TIMEOUT_LIBINDY_ERROR:
                return new TimeoutLibindyErrorException();
            case CREDENTIAL_DEF_ALREADY_CREATED:
                return new CredentialDefAlreadyCreatedException();
            case INVALID_SCHEMA_SEQ_NO:
                return new InvalidSchemaSeqNoException();
            case INVALID_SCHEMA_CREATION:
                return new InvalidSchemaCreationException();
            case INVALID_SCHEMA_HANDLE:
                return new InvalidSchemahandleException();
            case INVALID_MASTER_SECRET:
                return new InvalidMasterSecretException();
            case ALREADY_INITIALIZED:
                return new AlreadyInitializedException();
            case INVALID_INVITE_DETAILS:
                return new InvalidInviteDetailsException();
            case INVALID_OBJ_HANDLE:
                return new InvalidObjHandleException();
            case INVALID_DISCLOSED_PROOF_HANDLE:
                return new InvalidDisclosedProofHandleException();
            case SERIALIZATION_ERROR:
                return new SerializationErrorException();
            case WALLET_ALREADY_EXISTS:
                return new WalletAlreadyExistsException();
            case WALLET_ALREADY_OPEN:
                return new WalletAleradyOpenException();
            case WALLET_ITEM_NOT_FOUND:
                return new WalletItemNotFoundException();
            case WALLET_ITEM_CANNOT_ADD:
                return new WalletItemAlreadyExistsException();
            case INVALID_WALLET_HANDLE:
                return new InvalidWalletHandleException();
            case CANNOT_DELETE_CONNECTION:
                return new CannotDeleteConnectionException();
            case CREATE_CONNECTION_ERROR:
                return new CreateConnectionException();
            case INVALID_WALLET_CREATION:
                return new WalletCreateException();
            case INVALID_CREDENTIAL_HANDLE:
                return new InvalidCredentialHandleException();
            case INVALID_CREDENTIAL_JSON:
                return new InvalidCredentialJsonException();
            case CREATE_PROOF_ERROR:
                return new CreateProofErrorException();
            case INSUFFICIENT_TOKEN_AMOUNT:
                return new InsufficientTokenAmountException();
            case INVALID_PAYMENT_ADDRESS:
                return new InvalidPaymentAddressException();
            case ACTION_NOT_SUPPORTED:
                return new ActionNotSupportedException();
            case WALLET_ACCESS_FAILED:
                return new WalletAccessFailedException();
            case NO_AGENT_INFO:
                return new NoAgentInfoException();
            case INVALID_LIBINDY_PARAM:
                return new InvalidLibindyParamException();
            case MISSING_WALLET_KEY:
                return new MissingWalletKeyException();
            case OBJECT_CACHE_ERROR:
                return new ObjectCacheException();
            case NO_PAYMENT_INFORMATION:
                return new NoPaymentInformationException();
            case DUPLICATE_WALLET_RECORD:
                return new DuplicateWalletRecordException();
            case WALLET_RECORD_NOT_FOUND:
                return new WalletRecordNotFoundException();
            case INVALID_WALLET_IMPORT_CONFIG:
                return new InvalidWalletWalletImportConfigException();
            case MISSING_BACKUP_KEY:
                return new MissingBackupKeyException();
            case WALLET_NOT_FOUND:
                return new WalletNotFoundException();
            case LIBINDY_INVALID_STRUCTURE:
                return new LibindyInvalidStructureException();
            case INVALID_STATE:
                return new InvalidStateException();
            case INVALID_LEDGER_RESPONSE:
                return new InvalidLedgerResponseException();
            case DID_ALREADY_EXISTS_IN_WALLET:
                return new DidAlreadyExistsInWalletException();
            case DUPLICATE_MASTER_SECRET:
                return new DuplicateMasterSecretException();
            case INVALID_PROOF_REQUEST:
                return new InvalidProofRequestException();
            case IOERROR:
                return new IOException();
            case MISSING_PAYMENT_METHOD:
                return new MissingPaymentMethodException();
            case DUPLICATE_SCHEMA:
                return new DuplicateSchemaException();
            case LOGGING_ERROR:
                return new LoggingException();
            case INVALID_REVOCATION_DETAILS:
                return new InvalidRevocationDetailsException();
            case INVALID_REV_ENTRY:
                return new InvalidRevocationEntryException();
            case INVALID_REVOCATION_TIMESTAMP:
                return new InvalidRevocationTimestampException();
            case UNKNOWN_SCHEMA_REJECTION:
                return new UnknownSchemaRejectionException();
            case INVALID_REV_REG_DEF_CREATION:
                return new InvalidRevRegDefCreationException();
            case CREATE_WALLET_BACKUP:
                return new CreateWalletBackupException();
            case RETRIEVE_EXPORTED_WALLET:
                return new RetriveExportedWalletException();
            case RETRIEVE_DEAD_DROP:
                return new RetriveDeadDropException();
            case INVALID_ATTACHMENT_ENCODING:
                return new InvalidAttachementEncodingException();
            case INVALID_REDIRECT_DETAILS:
                return new InvalidRedirectDetailsException();
            case MAX_BACKUP_SIZE:
                return new MaxBackupSizeException();
            case INVALID_PROVISION_TOKEN:
                return new InvalidProvisionTokenException();
            case INVALID_DID_DOC:
                return new InvalidDidDocException();
            case MESSAGE_IS_OUT_OF_THREAD:
                return new MessageIsOutOfThreadException();
            case INVALID_AGENCY_REQUEST:
                return new InvalidAgencyRequestException();
            case CONNECTION_ALREADY_EXISTS:
                return new ConnectionAlreadyExistsException();
            case CONNECTION_DOES_NOT_EXIST:
                return new ConnectionDoesNotExistException();
            case INVALID_PROOF_PROPOSAL:
                return new InvalidProofProposalException();
            case UNIDENTIFIED_ERROR_CODE:
                String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
                return new VcxException(message, sdkErrorCode);
            default:
                String errMessage = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
                return new VcxException(errMessage, sdkErrorCode);
        }
    }
}


