namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when attempting to use a full revocation registry.
    /// </summary>
    public class RevocationRegistryFullException : IndyException
    {
        /// <summary>
        /// Initializes a new RevocationRegistryFullException.
        /// </summary>
        internal RevocationRegistryFullException(string message) : base(message, (int)ErrorCode.AnoncredsRevocationRegistryFullError)
        {

        }
    }

}
