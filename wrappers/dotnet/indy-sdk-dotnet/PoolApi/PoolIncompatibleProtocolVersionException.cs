namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when Pool Genesis Transactions are not compatible with Protocol version.
    /// </summary>
    public class PoolIncompatibleProtocolVersionException : IndyException
    {
        /// <summary>
        /// Initializes a new PoolIncompatibleProtocolVersionException.
        /// </summary>
        internal PoolIncompatibleProtocolVersionException(string message) : base(message, (int)ErrorCode.PoolIncompatibleProtocolVersionError)
        {

        }
    }

}
