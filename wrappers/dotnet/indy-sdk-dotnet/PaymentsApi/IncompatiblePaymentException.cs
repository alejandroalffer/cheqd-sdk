namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Information passed to libindy is incompatible.
    /// </summary>
    public class IncompatiblePaymentMethodsException : IndyException
    {
        internal IncompatiblePaymentMethodsException(string message) : base(message, (int)ErrorCode.PaymentIncompatibleMethodsError)
        {
        }
    }
}
