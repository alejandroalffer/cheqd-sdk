using System.Diagnostics;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that one of the parameters provided to an SDK call contained a valid that was considered invalid.
    /// </summary>
    public class InvalidParameterException : IndyException
    {
        /// <summary>
        /// Gets the index of the parameter from the SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The SDK error code.</param>
        /// <returns>The parameter index the SDK indicated was invalid.</returns>
        private static int GetParamIndex(int sdkErrorCode)
        {
            Debug.Assert((int)sdkErrorCode >= 100 && (int)sdkErrorCode <= 111);

            return (int)sdkErrorCode - 99;
        }

        /// <summary>
        /// Initializes a new InvalidParameterException from the specified SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The SDK error code that specifies which parameter was invalid.</param>
        internal InvalidParameterException(string message, int sdkErrorCode) : base(message, sdkErrorCode)
        {
            ParameterIndex = GetParamIndex(sdkErrorCode);
        }

        /// <summary>
        /// Gets the index of the parameter that contained the invalid value.
        /// </summary>
        public int ParameterIndex { get; private set; }
    }
}
