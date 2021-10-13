namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an attempt to create a master-secret with the same name as an existing master-secret.
    /// </summary>
    public class DuplicateMasterSecretNameException : IndyException
    {
        /// <summary>
        /// Initializes a new DuplicateMasterSecretNameException.
        /// </summary>
        internal DuplicateMasterSecretNameException(string message) : base(message, (int)ErrorCode.AnoncredsMasterSecretDuplicateNameError)
        {

        }
    }

}
