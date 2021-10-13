namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// An unknown payment method was called.
    /// </summary>
    public class UnknownPaymentMethodException : IndyException
    {
        internal UnknownPaymentMethodException(string message) : base(message, (int)ErrorCode.PaymentUnknownMethodError)
        {
        }
    }
}
