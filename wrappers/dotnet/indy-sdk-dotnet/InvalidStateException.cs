namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that the SDK library experienced an unexpected internal error.
    /// </summary>
    public class InvalidStateException : IndyException
    {
        /// <summary>
        /// Initializes a new InvalidStateException.
        /// </summary>
        internal InvalidStateException(string message) : base(message, (int)ErrorCode.CommonInvalidState)
        {
        }
    }

}
