namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when the no consensus was reached during a ledger operation.
    /// </summary>
    public class LedgerConsensusException : IndyException
    {
        /// <summary>
        /// Initializes a new LedgerConsensusException.
        /// </summary>
        internal LedgerConsensusException(string message) : base(message, (int)ErrorCode.LedgerNoConsensusError)
        {

        }
    }

}
