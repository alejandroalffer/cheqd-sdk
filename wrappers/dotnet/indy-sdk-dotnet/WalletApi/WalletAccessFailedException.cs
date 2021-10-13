namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet using invalid credentials.
    /// </summary>
    public class WalletAccessFailedException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletAccessFailedException.
        /// </summary>
        internal WalletAccessFailedException(string message) : base(message, (int)ErrorCode.WalletAccessFailed)
        {

        }
    }

}
