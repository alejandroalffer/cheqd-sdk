namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a proof has been rejected.
    /// </summary>
    public class ProofRejectedException : IndyException
    {
        /// <summary>
        /// Initializes a new ProofRejectedException.
        /// </summary>
        internal ProofRejectedException(string message) : base(message, (int)ErrorCode.AnoncredsProofRejected)
        {

        }
    }

}
