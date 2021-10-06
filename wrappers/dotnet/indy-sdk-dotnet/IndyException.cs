using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PaymentsApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using System;
using System.Runtime.InteropServices;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating a problem originating from the Indy SDK.
    /// </summary>
    public class IndyException : Exception
    {
        [DllImport(Consts.NATIVE_LIB_NAME)]
        internal static extern void indy_get_current_error(out IntPtr s);

        /// <summary>
        /// Retrieves the most recent Indy error message.
        /// </summary>
        internal static string GetCurrentError()
        {
            IntPtr s = IntPtr.Zero;
            indy_get_current_error(out s);
            if (s == IntPtr.Zero) 
            {
                // this should never happen, but just in case
                return "Error retrieving Indy error message";
            }
            else 
            {
                var json = Marshal.PtrToStringAnsi((IntPtr)s);
                var obj = JObject.Parse(json);
                return obj["message"].ToString();
            }
        }

        /// <summary>
        /// Initializes a new IndyException with the specified message and SDK error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="sdkErrorCode">The SDK error code for the exception.</param>
        internal IndyException(String message, int sdkErrorCode) : base(message)
        {
            SdkErrorCode = sdkErrorCode;
        }

        /// <summary>
        /// Generates an IndyException or one of its subclasses from the provided SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The error code.</param>
        /// <returns>An IndyException or subclass instance.</returns>
        internal static IndyException FromSdkError(int sdkErrorCode)
        {
            var errorCode = (ErrorCode)sdkErrorCode;
            var message = GetCurrentError();
            
            switch (errorCode)
            {
                case ErrorCode.CommonInvalidParam1:
                case ErrorCode.CommonInvalidParam2:
                case ErrorCode.CommonInvalidParam3:
                case ErrorCode.CommonInvalidParam4:
                case ErrorCode.CommonInvalidParam5:
                case ErrorCode.CommonInvalidParam6:
                case ErrorCode.CommonInvalidParam7:
                case ErrorCode.CommonInvalidParam8:
                case ErrorCode.CommonInvalidParam9:
                case ErrorCode.CommonInvalidParam10:
                case ErrorCode.CommonInvalidParam11:
                case ErrorCode.CommonInvalidParam12:
                case ErrorCode.CommonInvalidParam13:
                case ErrorCode.CommonInvalidParam14:
                case ErrorCode.CommonInvalidParam15:
                case ErrorCode.CommonInvalidParam16:
                case ErrorCode.CommonInvalidParam17:
                case ErrorCode.CommonInvalidParam18:
                case ErrorCode.CommonInvalidParam19:
                case ErrorCode.CommonInvalidParam20:
                case ErrorCode.CommonInvalidParam21:
                case ErrorCode.CommonInvalidParam22:
                case ErrorCode.CommonInvalidParam23:
                case ErrorCode.CommonInvalidParam24:
                case ErrorCode.CommonInvalidParam25:
                case ErrorCode.CommonInvalidParam26:
                case ErrorCode.CommonInvalidParam27:
                    return new InvalidParameterException(message, sdkErrorCode);
                case ErrorCode.CommonInvalidState:
                    return new InvalidStateException(message);
                case ErrorCode.CommonInvalidStructure:
                    return new InvalidStructureException(message);
                case ErrorCode.CommonIOError:
                    return new IOException(message);
                case ErrorCode.WalletInvalidHandle:
                    return new InvalidWalletException(message); 
                case ErrorCode.WalletUnknownTypeError:
                    return new UnknownWalletTypeException(message); 
                case ErrorCode.WalletTypeAlreadyRegisteredError:
                    return new DuplicateWalletTypeException(message);
                case ErrorCode.WalletAlreadyExistsError:
                    return new WalletExistsException(message);
                case ErrorCode.WalletNotFoundError:
                    return new WalletNotFoundException(message);
                case ErrorCode.WalletIncompatiblePoolError:
                    return new WrongWalletForPoolException(message);
                case ErrorCode.WalletAlreadyOpenedError:
                    return new WalletAlreadyOpenedException(message);
                case ErrorCode.WalletAccessFailed:
                    return new WalletAccessFailedException(message);
                case ErrorCode.PoolLedgerNotCreatedError:
                    return new PoolConfigNotCreatedException(message);
                case ErrorCode.PoolLedgerInvalidPoolHandle:
                    return new InvalidPoolException(message);
                case ErrorCode.PoolLedgerTerminated:
                    return new PoolLedgerTerminatedException(message);
                case ErrorCode.PoolIncompatibleProtocolVersionError:
                    return new PoolIncompatibleProtocolVersionException(message);
                case ErrorCode.PoolLedgerConfigAlreadyExistsError:
                    return new PoolLedgerConfigExistsException(message);
                case ErrorCode.LedgerNoConsensusError:
                    return new LedgerConsensusException(message);
                case ErrorCode.LedgerInvalidTransaction:
                    return new InvalidLedgerTransactionException(message);
                case ErrorCode.LedgerSecurityError:
                    return new LedgerSecurityException(message);
                case ErrorCode.AnoncredsRevocationRegistryFullError:
                    return new RevocationRegistryFullException(message);
                case ErrorCode.AnoncredsInvalidUserRevocId:
                    return new InvalidUserRevocIdException(message);
                case ErrorCode.AnoncredsMasterSecretDuplicateNameError:
                    return new DuplicateMasterSecretNameException(message);
                case ErrorCode.AnoncredsProofRejected:
                    return new ProofRejectedException(message);
                case ErrorCode.AnoncredsCredentialRevoked:
                    return new CredentialRevokedException(message);
                case ErrorCode.AnoncredsCredDefAlreadyExistsError:
                    return new CredentialDefinitionAlreadyExistsException(message);
                case ErrorCode.UnknownCryptoTypeError:
                    return new UnknownCryptoTypeException(message);
                case ErrorCode.WalletItemNotFoundError:
                    return new WalletItemNotFoundException(message);
                case ErrorCode.WalletItemAlreadyExistsError:
                    return new WalletItemAlreadyExistsException(message);
                case ErrorCode.WalletQueryError:
                    return new WalletInvalidQueryException(message);
                case ErrorCode.WalletStorageError:
                    return new WalletStorageException(message);
                case ErrorCode.WalletDecodingError:
                    return new WalletDecodingException(message);
                case ErrorCode.WalletEncryptionError:
                    return new WalletEncryptionException(message);
                case ErrorCode.WalletInputError:
                    return new WalletInputException(message);
                case ErrorCode.PaymentExtraFundsError:
                    return new ExtraFundsException(message);
                case ErrorCode.PaymentIncompatibleMethodsError:
                    return new IncompatiblePaymentMethodsException(message);
                case ErrorCode.PaymentInsufficientFundsError:
                    return new InsufficientFundsException(message);
                case ErrorCode.PaymentOperationNotSupportedError:
                    return new PaymentOperationNotSupportedException(message);
                case ErrorCode.PaymentSourceDoesNotExistError:
                    return new PaymentSourceDoesNotExistException(message);
                case ErrorCode.PaymentUnknownMethodError:
                    return new UnknownPaymentMethodException(message);

                default:
                    var defaultMessage = $"An unmapped error with the code '{sdkErrorCode}' was returned by the SDK.";
                    return new IndyException(defaultMessage, sdkErrorCode);
            }      
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int SdkErrorCode { get; private set; }
    }

}
