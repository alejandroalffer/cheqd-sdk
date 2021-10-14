namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet that does not exist.
    /// </summary>
    public class WalletNotFoundException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletNotFoundException.
        /// </summary>
        internal WalletNotFoundException(string message) : base(message, (int)ErrorCode.WalletNotFoundError)
        {

        }
    }

}
