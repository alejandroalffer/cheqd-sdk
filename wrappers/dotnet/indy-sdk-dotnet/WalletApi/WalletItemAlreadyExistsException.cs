namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when value with the specified key already exists in the wallet.
    /// </summary>
    /// <seealso cref="Hyperledger.Indy.IndyException" />
    public class WalletItemAlreadyExistsException : IndyException
    {
        internal WalletItemAlreadyExistsException(string message) : base(message, (int)ErrorCode.WalletItemAlreadyExistsError)
        {
        }
    }
}
