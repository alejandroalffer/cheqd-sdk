namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when registering a wallet type that has already been registered.
    /// </summary>
    public class DuplicateWalletTypeException : IndyException
    {
        /// <summary>
        /// Initializes a new DuplicateWalletTypeException.
        /// </summary>
        internal DuplicateWalletTypeException(string message) : base(message, (int)ErrorCode.WalletTypeAlreadyRegisteredError)
        {

        }
    }

}
