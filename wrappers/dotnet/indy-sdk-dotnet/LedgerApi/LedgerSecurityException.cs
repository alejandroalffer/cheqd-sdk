namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when attempting to send a transaction without the necessary privileges.
    /// </summary>
    public class LedgerSecurityException : IndyException
    {
        /// <summary>
        /// Initializes a new LedgerSecurityException.
        /// </summary>
        internal LedgerSecurityException(string message) : base(message, (int)ErrorCode.LedgerSecurityError)
        {

        }
    }

}
