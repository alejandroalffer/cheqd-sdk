namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that a value being processed was not considered a valid value.
    /// </summary>
    public class InvalidStructureException : IndyException
    {
        /// <summary>
        /// Initializes a new InvalidStructureException.
        /// </summary>
        internal InvalidStructureException(string message) : base(message, (int)ErrorCode.CommonInvalidStructure)
        {

        }
    }

}
