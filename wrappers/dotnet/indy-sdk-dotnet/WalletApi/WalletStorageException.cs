namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when a storage error occurs during a wallet operation.
    /// </summary>
    public class WalletStorageException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletStorageException.
        /// </summary>
        internal WalletStorageException(string message) : base(message, (int)ErrorCode.WalletStorageError)
        {

        }
    }

}
