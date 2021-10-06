namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when the input provided to a wallet operation is invalid.
    /// </summary>
    public class WalletInputException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletInputException.
        /// </summary>
        internal WalletInputException(string message) : base(message, (int)ErrorCode.WalletInputError)
        {

        }
    }

}
