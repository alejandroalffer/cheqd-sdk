namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet with a type that has not been registered.
    /// </summary>
    public class UnknownWalletTypeException : IndyException
    {
        /// <summary>
        /// Initializes a new UnknownWalletTypeException.
        /// </summary>
        internal UnknownWalletTypeException(string message) : base(message, (int)ErrorCode.WalletUnknownTypeError)
        {

        }
    }

}
