namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that an IO error occurred.
    /// </summary>
    public class IOException : IndyException
    {
        /// <summary>
        /// Initializes a new IOException.
        /// </summary>
        internal IOException(string message) : base(message, (int)ErrorCode.CommonIOError)
        {

        }
    }

}
