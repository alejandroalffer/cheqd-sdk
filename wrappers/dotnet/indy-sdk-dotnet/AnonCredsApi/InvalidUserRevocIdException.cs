namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an invalid user revocation registry id is used.
    /// </summary>
    public class InvalidUserRevocIdException : IndyException
    {
        /// <summary>
        /// Initializes a new InvalidUserRevocIdException.
        /// </summary>
        internal InvalidUserRevocIdException(string message) : base(message, (int)ErrorCode.AnoncredsInvalidUserRevocId)
        {

        }
    }

}
