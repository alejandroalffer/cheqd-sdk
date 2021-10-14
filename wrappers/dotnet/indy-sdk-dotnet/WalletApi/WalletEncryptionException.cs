namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when an error occurred during encryption-related operations.
    /// </summary>
    public class WalletEncryptionException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletEncryptionException.
        /// </summary>
        internal WalletEncryptionException(string message) : base(message, (int)ErrorCode.WalletEncryptionError)
        {

        }
    }

}
