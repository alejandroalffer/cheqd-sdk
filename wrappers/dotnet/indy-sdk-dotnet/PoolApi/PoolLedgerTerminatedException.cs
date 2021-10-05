namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when the pool ledger was terminated.
    /// </summary>
    public class PoolLedgerTerminatedException : IndyException
    {
        /// <summary>
        /// Initializes a new PoolLedgerTerminatedException.
        /// </summary>
        internal PoolLedgerTerminatedException(string message) : base(message, (int)ErrorCode.PoolLedgerTerminated)
        {

        }
    }

}
