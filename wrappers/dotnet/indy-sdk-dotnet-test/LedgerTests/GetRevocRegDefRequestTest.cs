using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class GetRevocRegDefRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildGetRevocRegDefRequestWorks()
        {
            var expectedResult =
                $"\"operation\":{{\"type\":\"115\",\"id\":\"{revRegDefId}\"}}";

            var request = await Ledger.BuildGetRevocRegDefRequestAsync(DID, revRegDefId);

            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }
    }
}
