namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// No such source found.
    /// </summary>
    public class PaymentSourceDoesNotExistException : IndyException
    {
        internal PaymentSourceDoesNotExistException(string message) : base(message, (int)ErrorCode.PaymentSourceDoesNotExistError)
        {
        }
    }
}
