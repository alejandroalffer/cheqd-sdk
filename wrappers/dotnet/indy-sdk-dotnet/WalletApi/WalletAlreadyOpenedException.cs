namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet that was already opened.
    /// </summary>
    public class WalletAlreadyOpenedException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletAlreadyOpenedException.
        /// </summary>
        internal WalletAlreadyOpenedException(string message) : base(message, (int)ErrorCode.WalletAlreadyOpenedError)
        {

        }
    }

}
