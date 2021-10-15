namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Exception thrown when an unknown crypto format is used for DID entity keys.
    /// </summary>
    public class UnknownCryptoTypeException : IndyException
    {
        /// <summary>
        /// Initializes a new UnknownCryptoTypeException.
        /// </summary>
        internal UnknownCryptoTypeException(string message) : base(message, (int)ErrorCode.UnknownCryptoTypeError)
        {

        }
    }

}
