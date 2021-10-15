namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when value with the specified key doesn't exists in the wallet from which it was requested.
    /// </summary>
    /// <seealso cref="Hyperledger.Indy.IndyException" />
    public class WalletItemNotFoundException : IndyException
    {
        internal WalletItemNotFoundException(string message) : base(message, (int)ErrorCode.WalletItemNotFoundError)
        {
        }
    }
}
