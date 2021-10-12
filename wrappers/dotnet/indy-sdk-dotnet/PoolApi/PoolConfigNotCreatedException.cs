namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to open pool which does not yet have a created configuration.
    /// </summary>
    public class PoolConfigNotCreatedException : IndyException
    {
        /// <summary>
        /// Initializes a new PoolConfigNotCreatedException.
        /// </summary>
        internal PoolConfigNotCreatedException(string message) : base(message, (int)ErrorCode.PoolLedgerNotCreatedError)
        {

        }
    }

}
