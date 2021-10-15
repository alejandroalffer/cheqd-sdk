namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when creating a wallet and a wallet with the same name already exists.
    /// </summary>
    public class WalletExistsException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletExistsException.
        /// </summary>
        internal WalletExistsException(string message) : base(message, (int)ErrorCode.WalletAlreadyExistsError)
        {

        }
    }

}
