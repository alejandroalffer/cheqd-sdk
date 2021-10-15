namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Extra funds on inputs.
    /// </summary>
    public class ExtraFundsException : IndyException
    {
        internal ExtraFundsException(string message) : base(message, (int)ErrorCode.PaymentExtraFundsError)
        {
        }
    }
}
