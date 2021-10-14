namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Operation is not supported for payment method.
    /// </summary>
    public class PaymentOperationNotSupportedException : IndyException
    {
        internal PaymentOperationNotSupportedException(string message) : base(message, (int)ErrorCode.PaymentOperationNotSupportedError)
        {
        }
    }
}
