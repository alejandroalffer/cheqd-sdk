namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a credential has been revoked.
    /// </summary>
    public class CredentialRevokedException : IndyException
    {
        /// <summary>
        /// Initializes a new CredentialRevokedException.
        /// </summary>
        internal CredentialRevokedException(string message) : base(message, (int)ErrorCode.AnoncredsCredentialRevoked)
        {

        }
    }

}
