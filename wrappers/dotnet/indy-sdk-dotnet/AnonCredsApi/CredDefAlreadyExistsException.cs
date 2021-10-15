namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when attempting create a credential definition that already exists.
    /// </summary>
    public class CredentialDefinitionAlreadyExistsException : IndyException
    {
        /// <summary>
        /// Initializes a new CredDefAlreadyExistsException.
        /// </summary>
        internal CredentialDefinitionAlreadyExistsException(string message) : base(message, (int)ErrorCode.AnoncredsCredDefAlreadyExistsError)
        {

        }
    }

}
