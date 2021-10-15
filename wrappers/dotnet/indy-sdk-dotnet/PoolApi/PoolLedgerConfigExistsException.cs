namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to create a pool ledger config with same name as an existing pool ledger config.
    /// </summary>
    public class PoolLedgerConfigExistsException : IndyException
    {
        /// <summary>
        /// Initializes a new PoolLedgerConfigExistsException.
        /// </summary>
        internal PoolLedgerConfigExistsException(string message) : base(message, (int)ErrorCode.PoolLedgerConfigAlreadyExistsError)
        {

        }
    }

}
