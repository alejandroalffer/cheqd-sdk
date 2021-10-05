namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to use a wallet with the wrong pool.
    /// </summary>
    public class WalletInvalidQueryException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletInvalidQueryException.
        /// </summary>
        internal WalletInvalidQueryException(string message) : base(message, (int)ErrorCode.WalletQueryError)
        {

        }
    }

}
