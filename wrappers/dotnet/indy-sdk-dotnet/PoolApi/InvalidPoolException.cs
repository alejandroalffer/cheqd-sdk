namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to use a pool that has been closed or is invalid.
    /// </summary>
    public class InvalidPoolException : IndyException
    {
        /// <summary>
        /// Initializes a new PoolClosedException.
        /// </summary>
        internal InvalidPoolException(string message) : base(message, (int)ErrorCode.PoolLedgerInvalidPoolHandle)
        {

        }
    }

}
