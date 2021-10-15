namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Insufficient funds on inputs.
    /// </summary>
    public class InsufficientFundsException : IndyException
    {
        internal InsufficientFundsException(string message) : base(message, (int)ErrorCode.PaymentInsufficientFundsError)
        {
        }
    }
}
