namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when decoding of wallet data during input/output failed.
    /// </summary>
    public class WalletDecodingException : IndyException
    {
        /// <summary>
        /// Initializes a new WalletDecodingException.
        /// </summary>
        internal WalletDecodingException(string message) : base(message, (int)ErrorCode.WalletDecodingError)
        {

        }
    }

}
