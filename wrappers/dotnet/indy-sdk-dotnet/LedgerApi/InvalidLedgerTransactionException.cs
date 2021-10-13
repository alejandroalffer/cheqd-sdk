namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when attempting to send an unknown or incomplete ledger message.
    /// </summary>
    public class InvalidLedgerTransactionException : IndyException
    {
        /// <summary>
        /// Initializes a new InvalidLedgerTransactionException.
        /// </summary>
        internal InvalidLedgerTransactionException(string message) : base(message, (int)ErrorCode.LedgerInvalidTransaction)
        {

        }
    }

}
