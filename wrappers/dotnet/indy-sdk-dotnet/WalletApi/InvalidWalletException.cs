namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when an attempt is made to use a closed or invalid wallet.
    /// </summary>
    public class InvalidWalletException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletClosedException.
        /// </summary>
        internal InvalidWalletException(string message) : base(message, (int)ErrorCode.WalletInvalidHandle)
        {

        }
    }

}
