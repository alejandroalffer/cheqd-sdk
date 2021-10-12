using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class DisposePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task CanDisposeClosedPool()
        {
            var poolName = await PoolUtils.CreatePoolLedgerConfig();

            using (var pool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
                await pool.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var poolName = await PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();
            pool.Dispose();
        }

        [TestMethod]
        public async Task ClosingDisposedPoolStillProvidesSDKError()
        {
            var poolName = await PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<InvalidPoolException>(() =>
                pool.CloseAsync()
            );
        }      
    }
}
