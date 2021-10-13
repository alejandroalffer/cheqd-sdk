namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to use a wallet with the wrong pool.
    /// </summary>
    public class WrongWalletForPoolException : IndyException
    {
        /// <summary>
        /// Initializes a new WrongWalletForPoolException.
        /// </summary>
        internal WrongWalletForPoolException(string message) : base(message, (int)ErrorCode.WalletIncompatiblePoolError)
        {

        }
    }

}
